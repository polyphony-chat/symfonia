// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use sqlx_pg_uint::PgU64;

use crate::errors::Error;

#[derive(Debug, Clone, FromRow)]
struct PgU64Mapper {
	inner: BigDecimal,
}

impl PgU64Mapper {
	fn into_pg_u64(self) -> Result<PgU64, sqlx_pg_uint::Error> {
		PgU64::try_from(self.inner)
	}
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserSettings {
	#[sqlx(flatten)]
	#[serde(flatten)]
	inner: chorus::types::UserSettings,
	pub index: PgU64,
}

impl Deref for UserSettings {
	type Target = chorus::types::UserSettings;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for UserSettings {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl UserSettings {
	pub fn as_inner(&self) -> &chorus::types::UserSettings {
		&self.inner
	}

	pub fn as_inner_mut(&mut self) -> &mut chorus::types::UserSettings {
		&mut self.inner
	}

	pub fn into_inner(self) -> chorus::types::UserSettings {
		self.inner
	}

	pub fn consume(inner: chorus::types::UserSettings, index: u64) -> Self {
		Self { inner, index: PgU64::from(index) }
	}

	pub async fn create(db: &PgPool, locale: &str) -> Result<Self, Error> {
		let mut settings = Self {
			inner: chorus::types::UserSettings { locale: locale.to_string(), ..Default::default() },
			index: PgU64::from(0),
		};

		let res: PgU64Mapper = sqlx::query_as(
			"INSERT INTO user_settings (locale) VALUES ($1) RETURNING index as inner",
		)
		.bind(locale)
		.fetch_one(db)
		.await?;
		let index = res.into_pg_u64()?;
		settings.index = index;
		Ok(settings)
	}

	pub async fn get_by_index(db: &PgPool, index: u64) -> Result<UserSettings, Error> {
		let index = PgU64::from(index);
		sqlx::query_as("SELECT * FROM user_settings WHERE index = ?")
			.bind(index)
			.fetch_one(db)
			.await
			.map_err(Error::Sqlx)
	}
}
