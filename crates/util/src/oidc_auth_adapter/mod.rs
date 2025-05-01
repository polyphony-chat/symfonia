// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/*
Create a confidential OIDC client which can create, read, update and delete OIDC users.
This client will serve as a "bridge" for Spacebar API clients, so that they can
authenticate even though they have no clue about OIDC.
*/

use bigdecimal::BigDecimal;
use chorus::types::{RegisterSchema, Snowflake, UserModifySchema};
use sqlx::{PgPool, postgres::PgValue};

use crate::{
	entities::{Config, User},
	errors,
};

/// Represents a set of administration APIs of an OIDC IDP needed to register a
/// new user. There is no proper, standardized administration API for IDPs, and
/// registering a new client programmatically is not a part of OIDC. IDPs like
/// "Rauthy" will provide an admin API to do this.
pub trait AdminApi {
	/// The user type specific to this admin API implementation
	type User;
	type Error: std::error::Error;

	/// Register a new OIDC user using this admin API implementation.
	///
	/// ## Parameters
	///
	/// - `register_schema` [RegisterSchema]: Registration information provided
	///   by the Spacebar client.
	/// - `client_ip` [str]: IP of the client; MUST be forwarded as `X-Real-Ip`
	///   header to make use of security features.
	fn register_user(
		login_schema: &RegisterSchema,
		client_ip: &str,
	) -> impl std::future::Future<Output = Result<Self::User, Self::Error>> + Send;

	/// Edit a OIDC user using this admin API implementation.
	///
	/// ## Parameters
	///
	/// - `register_schema` [RegisterSchema]: Registration information provided
	///   by the Spacebar client.
	/// - `client_ip` [str]: IP of the client; MUST be forwarded as `X-Real-Ip`
	///   header to make use of security features.
	fn edit_user(
		modify_schema: &UserModifySchema,
		client_ip: &str,
	) -> impl std::future::Future<Output = Result<Self::User, Self::Error>> + Send;

	/// Delete a OIDC user using this admin API implementation.
	///
	/// ## Parameters
	///
	/// - `client_ip` [str]: IP of the client; MUST be forwarded as `X-Real-Ip`
	///   header to make use of security features.
	fn delete_user(
		oidc_sub: &str,
		client_ip: &str,
	) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

	/// Retrieve the OIDC `sub` attribute of a `Self::User`
	fn user_oid_sub(user: &Self::User) -> String;
}

/// Insert a new adapter auth adapter user into the database. Creates a new
/// entry in the `users` table, and an entry in the `oidc_spacebar` table,
/// mapping the created user to a OIDC `sub` value.
async fn insert_adapter_user(
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
	let _oidc_sub = sqlx::query_as!(
		String,
		r#"
        INSERT INTO oidc_spacebar (oidc_sub, user_id)
        VALUES ($1, $2)
    "#,
		oidc_sub,
		BigDecimal::from(u64::from(user.id))
	)
	.execute(pool)
	.await?;
	Ok(user)
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Either/Or-style argument to pass to the [delete_adapter_user] method.
pub enum DeleteInfo {
	/// A [Snowflake] ID
	IdSnowflake(Snowflake),
	/// An OIDC sub (primary key)
	OidcSub(String),
}

/// Deletes an adapter user from the adapter user table. Does *not* delete the
/// user from the Users table. Depending on the supplied [DeleteInfo], this will
/// delete either all entries in the adapter user table where the snowflake
/// matches the supplied snowflake, OR the entry where the oidc_sub primary key
/// matches the supplied [DeleteInfo::OidcSub].
///
/// If you intend to delete the user from that table as well, make sure to
/// preserve information about the OIDC sub<-->Snowflake ID mapping.
async fn delete_adapter_user(
	pool: &PgPool,
	delete_info: DeleteInfo,
) -> Result<(), crate::errors::Error> {
	sqlx::query!(
		r#"
        DELETE FROM oidc_spacebar 
		WHERE oidc_sub = $1 OR user_id = $2
    	"#,
		match &delete_info {
			DeleteInfo::IdSnowflake(_) => String::new(),
			DeleteInfo::OidcSub(value) => value.clone(),
		},
		match &delete_info {
			DeleteInfo::IdSnowflake(snowflake) => BigDecimal::from(u64::from(*snowflake)),
			DeleteInfo::OidcSub(_) => BigDecimal::default(),
		}
	)
	.execute(pool)
	.await?;
	Ok(())
}
