// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
	default::Default,
	ops::{Deref, DerefMut},
};

use argon2::{
	Argon2,
	password_hash::{self, PasswordHasher, SaltString},
};
use bigdecimal::BigDecimal;
use chorus::types::{PublicUser, Rights, Snowflake, UserData};
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, from_str};
use sqlx::{FromRow, PgPool, Row};
use sqlx_pg_uint::{PgU32, PgU64};

use super::*;
use crate::{
	entities::{Config, Guild, GuildMember, UserSettings},
	errors::{Error, GuildError},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct User {
	#[sqlx(flatten)]
	#[serde(flatten)]
	inner: chorus::types::User,
	pub data: sqlx::types::Json<UserData>,
	pub deleted: bool,
	pub fingerprints: String, // TODO: Simple-array, should actually be a vec
	pub settings_index: PgU64,
	pub rights: Rights,
	#[sqlx(skip)]
	pub settings: UserSettings,
	pub extended_settings: sqlx::types::Json<Value>,
	#[sqlx(skip)]
	#[serde(skip)]
	pub publisher: SharedEventPublisher,
}

impl Deref for User {
	type Target = chorus::types::User;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for User {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl User {
	#[allow(clippy::too_many_arguments)]
	pub async fn create(
		db: &PgPool,
		cfg: &Config,
		username: &str,
		password: Option<String>,
		email: Option<String>,
		fingerprint: Option<String>,
		date_of_birth: Option<NaiveDate>,
		bot: bool,
	) -> Result<Self, Error> {
		// TODO: trim username
		// TODO: generate discrim

		// TODO: dynamically figure out locale
		let user_settings = UserSettings::create(db, "en-US").await?;

		let argon2 = Argon2::default();
		let salt = SaltString::generate(password_hash::rand_core::OsRng);
		let cooked_password = match password {
			Some(password) => Some(argon2.hash_password(password.as_bytes(), &salt)?.to_string()),
			None => None,
		};

		let user_id = Snowflake::default();

		let user = Self {
			inner: chorus::types::User {
				username: username.to_string(),
				discriminator: "0000".to_string(),
				email: email.clone(),
				premium: cfg.defaults.user.premium.into(),
				premium_type: Some(cfg.defaults.user.premium_type),
				bot: Some(bot),
				verified: cfg.defaults.user.verified.into(),
				..Default::default()
			},
			data: sqlx::types::Json(UserData {
				hash: cooked_password,
				valid_tokens_since: Utc::now(),
			}),
			fingerprints: fingerprint.unwrap_or_default(),
			rights: cfg.register.default_rights,
			settings_index: user_settings.index.clone(),
			extended_settings: sqlx::types::Json(Value::Object(Map::default())),
			settings: user_settings.clone(),
			..Default::default()
		};

		let data: Value = from_str(&user.data.encode_to_string()?)?;
		let rights = PgU64::from(Rights::default().bits()).as_big_decimal().to_owned();

		sqlx::query("INSERT INTO users (id, username, discriminator, email, data, fingerprints, premium, premium_type, created_at, flags, public_flags, purchased_flags, premium_usage_flags, rights, extended_settings, settings_index) VALUES ($1, $2, $3, $4, $5, $6, false, 0, $7, 0, 0, 0, 0, $8, '{}', $9)")
            .bind(bigdecimal::BigDecimal::from(user.id.to_string().parse::<u64>().unwrap()))
            .bind(username)
            .bind(   "0000")
            .bind( email)
            .bind(  data)
            .bind(&user.fingerprints)
            .bind( Utc::now().naive_local())
            .bind(  Some(rights))
            .bind( user.settings_index.clone().as_big_decimal().to_owned())
            .execute(db)
            .await?;

		Ok(user)
	}

	async fn find_unused_discriminator(db: &PgPool, cfg: &Config) -> Result<String, Error> {
		// TODO: intelligently find unused discriminator: https://dba.stackexchange.com/questions/48594/find-numbers-not-used-in-a-column
		todo!()
	}

	pub async fn get_by_id(db: &PgPool, id: Snowflake) -> Result<Option<Self>, Error> {
		sqlx::query_as("SELECT * FROM users WHERE id = $1")
			.bind(id)
			.fetch_optional(db)
			.await
			.map_err(Error::Sqlx)
	}

	pub async fn get_by_id_list(
		db: &PgPool,
		ids: &[Snowflake],
		after: Option<Snowflake>,
		limit: PgU32,
	) -> Result<Vec<Self>, Error> {
		let mut query_builder = sqlx::QueryBuilder::new("SELECT * FROM users WHERE id IN (");
		let mut separated = query_builder.separated(", ");
		for id in ids {
			separated.push_bind(id);
		}
		separated.push_unseparated(") ");

		if let Some(after) = after {
			separated.push_unseparated("AND id > $1 ");
			separated.push_bind_unseparated(after);
		}
		separated.push_unseparated("LIMIT $2");
		separated.push_bind_unseparated(limit);

		let query = query_builder.build();

		let r = query.fetch_all(db).await.map_err(Error::Sqlx)?;
		let users = r.iter().map(User::from_row).map_while(|u| u.ok()).collect::<Vec<_>>();

		Ok(users)
	}

	pub async fn find_by_user_and_discrim(
		db: &PgPool,
		user: &str,
		discrim: &str,
	) -> Result<Option<Self>, Error> {
		sqlx::query_as("SELECT * FROM users WHERE username = $1 AND discriminator = $2")
			.bind(user)
			.bind(discrim)
			.fetch_optional(db)
			.await
			.map_err(Error::Sqlx)
	}

	pub async fn get_user_by_email_or_phone(
		db: &PgPool,
		email: &str,
		phone: &str,
	) -> Result<Option<Self>, Error> {
		sqlx::query_as("SELECT * FROM users WHERE email = $1 OR phone = $2 LIMIT 1")
			.bind(email)
			.bind(phone)
			.fetch_optional(db)
			.await
			.map_err(Error::Sqlx)
	}

	pub async fn add_to_guild(
		&self,
		db: &PgPool,
		guild_id: Snowflake,
	) -> Result<GuildMember, Error> {
		let public = self.to_public_user();

		let guild =
			Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

		GuildMember::create(db, self, &guild).await
	}

	/// Return the Snowflake IDs of all guilds the user is a member of. Limited
	/// to 1000 results to avoid memory exhaustion.
	pub async fn get_guild_ids(&self, db: &PgPool) -> Result<Vec<Snowflake>, Error> {
		let pg_u64s: Vec<PgU64> =
			sqlx::query_as("SELECT guild_id FROM members where id = $1 LIMIT $2")
				.bind(BigDecimal::from(u64::from(self.id)))
				.bind(10000)
				.fetch_all(db)
				.await
				.map_err(Error::Sqlx)?;
		Ok(pg_u64s.iter().map(|x| Snowflake::from(x.to_uint())).collect())
	}

	pub async fn count(db: &PgPool) -> Result<i32, Error> {
		sqlx::query("SELECT COUNT(*) FROM users")
			.fetch_one(db)
			.await
			.map_err(Error::Sqlx)
			.map(|r| r.get::<i32, _>(0))
	}

	pub async fn count_guilds(&self, db: &PgPool) -> Result<i32, Error> {
		GuildMember::count_by_user_id(db, self.id).await
	}

	pub fn to_public_user(&self) -> PublicUser {
		self.to_inner().into_public_user()
	}

	pub fn to_inner(&self) -> chorus::types::User {
		self.inner.clone()
	}

	// TODO: Implement this
	pub async fn get_relationships(
		target: Snowflake,
		db: &PgPool,
	) -> Result<Vec<Snowflake>, Error> {
		todo!()
	}
}

// TODO: move these back to symfonia
// #[cfg(test)]
// mod user_unit_tests {
//     use bigdecimal::BigDecimal;
//     use chorus::types::Snowflake;
//     use sqlx::PgPool;

//     use crate::entities::User;
//     #[sqlx::test(fixtures(path = "../../../fixtures", scripts("users",
// "guilds")))]     async fn get_user_guilds(pool: PgPool) {
//         sqlx::query!(
//             "INSERT INTO members(
//             id, guild_id, deaf, mute, pending, settings, bio, pronouns,
// joined_at)             VALUES ($1, $2, $3, $4, $5, $6, $7, $8,
// current_timestamp);",             BigDecimal::from(7248639845155737600_u64),
//             BigDecimal::from(7249086638293258240_u64),
//             false,
//             false,
//             false,
//             "{}",
//             "my bio on this guild",
//             "any/all"
//         )
//         .execute(&pool)
//         .await
//         .unwrap();
//         sqlx::query!(
//             "INSERT INTO members(
//             id, guild_id, deaf, mute, pending, settings, bio, pronouns,
// joined_at)             VALUES ($1, $2, $3, $4, $5, $6, $7, $8,
// current_timestamp);",             BigDecimal::from(7248639845155737600_u64),
//             BigDecimal::from(7249112309484752896_u64),
//             false,
//             false,
//             false,
//             "{}",
//             "my bio on this guild",
//             "any/all"
//         )
//         .execute(&pool)
//         .await
//         .unwrap();
//         let user = User::get_by_id(&pool,
// Snowflake::from(7248639845155737600))             .await
//             .unwrap()
//             .unwrap();
//         let guilds = user.get_guild_ids(&pool).await.unwrap();
//         assert_eq!(
//             guilds,
//             vec![
//                 Snowflake::from(7249086638293258240),
//                 Snowflake::from(7249112309484752896)
//             ]
//         );
//     }
// }
