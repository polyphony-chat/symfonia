// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/*
Create a confidential OIDC client which can create, read, update and delete OIDC users.
This client will serve as a "bridge" for Spacebar API clients, so that they can
authenticate even though they have no clue about OIDC.
*/

use bigdecimal::BigDecimal;
use chorus::types::{LoginSchema, RegisterSchema, Snowflake, UserModifySchema};
use sqlx::{
	PgPool,
	postgres::{PgRow, PgValue},
	query_as, query_scalar,
	types::Text,
};
use sqlx_pg_uint::PgU64;

use crate::{
	entities::{Config, User},
	errors,
};

/// Represents a set of administration APIs needed to register a
/// new user. In the context of symfonia, this is intended to offer backwards
/// compatibility for non-OIDC clients
pub trait AdminApi {
	/// The user type specific to this admin API implementation
	type User;
	type Error: std::error::Error;

	/// Register a new user using this admin API implementation.
	///
	/// ## Parameters
	///
	/// - `register_schema` [RegisterSchema]: Registration information provided
	///   by the Spacebar client.
	/// - `client_ip` [str]: IP of the client; MUST be forwarded as `X-Real-Ip`
	///   header to make use of security features, if an external auth provider
	///   is used.
	fn register_user(
		login_schema: &RegisterSchema,
		client_ip: &str,
		pool: &PgPool,
	) -> impl std::future::Future<Output = Result<Self::User, Self::Error>> + Send;

	/// Edit a user using this admin API implementation.
	///
	/// ## Parameters
	///
	/// - `register_schema` [RegisterSchema]: Registration information provided
	///   by the Spacebar client.
	/// - `client_ip` [str]: IP of the client; MUST be forwarded as `X-Real-Ip`
	///   header to make use of security features, if an external auth provider
	///   is used.
	fn edit_user(
		modify_schema: &UserModifySchema,
		client_ip: &str,
		pool: &PgPool,
	) -> impl std::future::Future<Output = Result<Self::User, Self::Error>> + Send;

	/// Delete a user using this admin API implementation.
	///
	/// ## Parameters
	///
	/// - `client_ip` [str]: IP of the client; MUST be forwarded as `X-Real-Ip`
	///   header to make use of security features, if an external auth provider
	///   is used.
	fn delete_user(
		oidc_sub: &str,
		client_ip: &str,
		pool: &PgPool,
	) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

	/// Retrieve the OIDC `sub` attribute of a `Self::User`
	fn user_oid_sub(user: &Self::User) -> String;
}

/// Insert a new adapter auth adapter user into the database. Creates a new
/// entry in the `users` table, and an entry in the `oidc_spacebar` table,
/// mapping the created user to a OIDC `sub` value.
pub async fn insert_adapter_user(
	pool: &PgPool,
	oidc_sub: &str,
	register_schema: &RegisterSchema,
	bot: bool,
) -> Result<User, crate::errors::Error> {
	let user = User::create(
		pool,
		&Config::init(pool).await?,
		&register_schema.username,
		register_schema.password.clone(),
		register_schema.email.clone(),
		register_schema.fingerprint.clone(),
		register_schema.date_of_birth,
		bot,
	)
	.await?;
	add_adapter_mapping(pool, oidc_sub, user.id).await?;
	Ok(user)
}

/// Adds a mapping from [User] to an `oidc_sub` value to the `oidc_spacebar`
/// table without creating a new [User]. Will fail, if the user specified by
/// `user_id` does not exist in the `users` table.
pub async fn add_adapter_mapping(
	pool: &PgPool,
	oidc_sub: &str,
	user_id: Snowflake,
) -> Result<(), crate::errors::Error> {
	sqlx::query!(
		r#"
		INSERT INTO oidc_spacebar (oidc_sub, user_id)
		VALUES ($1, $2)
	"#,
		oidc_sub,
		BigDecimal::from(user_id.0)
	)
	.execute(pool)
	.await
	.map_err(|e| e.into())
	.map(|_| ())
}

/// Retrieve a mapping from [User] to an `oidc_sub` value from the
/// `oidc_spacebar` table. Will fail, if the user specified by
/// [UserInfo] does not exist in the `users` table.
///
/// ## Returns
///
/// - If [UserInfo::IdSnowflake] was specified and the user specified by that
///   IdSnowflake exists in the database, returns the corresponding
///   [UserInfo::OidcSub].
/// - If [UserInfo::OidcSub] was specified and the user specified by that
///   OidcSub exists in the database, returns the corresponding
///   [UserInfo::IdSnowflake].
/// - If the user doesn't exist, returns an error.
pub async fn get_mapping(
	pool: &PgPool,
	user_info: &UserInfo,
) -> Result<UserInfo, crate::errors::Error> {
	match user_info {
		UserInfo::IdSnowflake(snowflake) => {
			let oidc_sub = query_as!(
				UserInfoReturn,
				r#"
			SELECT * FROM oidc_spacebar
			WHERE user_id = $1 LIMIT 1
			"#,
				BigDecimal::from(snowflake.0)
			)
			.fetch_one(pool)
			.await?;
			match oidc_sub.oidc_sub {
				Some(s) => Ok(UserInfo::OidcSub(s)),
				None => unreachable!(),
			}
		}
		UserInfo::OidcSub(oidc_sub) => {
			let user_id = query_as!(
				UserInfoReturn,
				r#"
			SELECT * FROM oidc_spacebar
			WHERE oidc_sub = $1 LIMIT 1
			"#,
				oidc_sub
			)
			.fetch_one(pool)
			.await?;
			match user_id.user_id {
				Some(b) => Ok(UserInfo::IdSnowflake(PgU64::try_from(b)?.to_uint().into())),
				None => unreachable!(),
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Either/Or-style argument to pass to the [delete_adapter_user] method.
pub enum UserInfo {
	/// A [Snowflake] ID
	IdSnowflake(Snowflake),
	/// An OIDC sub (primary key)
	OidcSub(String),
}

#[derive(Debug)]
struct UserInfoReturn {
	oidc_sub: Option<String>,
	user_id: Option<BigDecimal>,
}

/// Deletes an adapter user from the adapter user table. Does *not* delete the
/// user from the Users table. Depending on the supplied [UserInfo], this will
/// delete either all entries in the adapter user table where the snowflake
/// matches the supplied snowflake, OR the entry where the oidc_sub primary key
/// matches the supplied [UserInfo::OidcSub].
///
/// If you intend to delete the user from that table as well, make sure to
/// preserve information about the OIDC sub<-->Snowflake ID mapping.
async fn delete_adapter_user(
	pool: &PgPool,
	delete_info: UserInfo,
) -> Result<(), crate::errors::Error> {
	sqlx::query!(
		r#"
        DELETE FROM oidc_spacebar 
		WHERE oidc_sub = $1 OR user_id = $2
    	"#,
		match &delete_info {
			UserInfo::IdSnowflake(_) => String::new(),
			UserInfo::OidcSub(value) => value.clone(),
		},
		match &delete_info {
			UserInfo::IdSnowflake(snowflake) => BigDecimal::from(u64::from(*snowflake)),
			UserInfo::OidcSub(_) => BigDecimal::default(),
		}
	)
	.execute(pool)
	.await?;
	Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
	use super::*;
	use crate::init_logger;

	#[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
	async fn create_delete_adapter_mapping(pool: PgPool) {
		init_logger();
		add_adapter_mapping(
			&pool,
			"123e4567-e89b-12d3-a456-426614174000",
			Snowflake(7248639845155737600),
		)
		.await
		.unwrap();
		delete_adapter_user(&pool, UserInfo::IdSnowflake(Snowflake(7248639845155737600)))
			.await
			.unwrap();
		add_adapter_mapping(
			&pool,
			"123e4567-e89b-12d3-a456-426614174000",
			Snowflake(7248639845155737600),
		)
		.await
		.unwrap();
		delete_adapter_user(
			&pool,
			UserInfo::OidcSub("123e4567-e89b-12d3-a456-426614174000".into()),
		)
		.await
		.unwrap();
	}

	#[sqlx::test]
	async fn test_insert_adapter_user(pool: PgPool) {
		init_logger();
		let register_schema = RegisterSchema {
			username: String::from("usery_name"),
			password: None,
			consent: true,
			email: None,
			fingerprint: None,
			invite: None,
			date_of_birth: None,
			gift_code_sku_id: None,
			captcha_key: None,
			promotional_email_opt_in: None,
		};
		let user = insert_adapter_user(
			&pool,
			"123e4567-e89b-12d3-a456-426614174000",
			&register_schema,
			false,
		)
		.await
		.unwrap();

		let user_info = UserInfo::IdSnowflake(Snowflake(user.id.into()));
		let result = get_mapping(&pool, &user_info).await.unwrap();
		assert_eq!(result, UserInfo::OidcSub("123e4567-e89b-12d3-a456-426614174000".into()));

		let user_info = UserInfo::OidcSub(String::from("123e4567-e89b-12d3-a456-426614174000"));
		let result = get_mapping(&pool, &user_info).await.unwrap();
		assert_eq!(result, UserInfo::IdSnowflake(Snowflake(user.id.into())));
	}

	#[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
	async fn test_get_mapping_nonexistent_id(pool: PgPool) {
		init_logger();
		let user_info = UserInfo::IdSnowflake(Snowflake(9999999999999999999));
		let result = get_mapping(&pool, &user_info).await;
		assert!(result.is_err());
	}

	#[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
	async fn test_get_mapping_nonexistent_oidc_sub(pool: PgPool) {
		init_logger();
		let user_info = UserInfo::OidcSub(String::from("nonexistent-oidc-sub"));
		let result = get_mapping(&pool, &user_info).await;
		assert!(result.is_err());
	}
}
