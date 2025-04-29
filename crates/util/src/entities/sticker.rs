// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

use chorus::types::{Snowflake, StickerFormatType, StickerType};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};

use crate::{entities::User, errors::Error};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Sticker {
	#[sqlx(flatten)]
	inner: chorus::types::Sticker,
	pub user_id: Option<Snowflake>,
}

impl Deref for Sticker {
	type Target = chorus::types::Sticker;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for Sticker {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl Sticker {
	pub async fn create(
		db: &PgPool,
		guild_id: Option<Snowflake>,
		pack_id: Option<Snowflake>,
		user_id: Option<Snowflake>,
		name: &str,
		description: Option<String>,
		tags: Option<String>,
		sticker_type: StickerType,
		sticker_format_type: StickerFormatType,
	) -> Result<Self, Error> {
		let id = Snowflake::generate();
		sqlx::query("INSERT INTO stickers (id, guild_id, pack_id, user_id, name, description, tags, `type`, format_type) VALUES (?,?,?,?,?,?,?,?,?)")
            .bind(id)
            .bind(guild_id)
            .bind(pack_id)
            .bind(user_id)
            .bind(name)
            .bind(&description)
            .bind(&tags)
            .bind(sticker_type)
            .bind(sticker_format_type)
            .execute(db)
            .await?;

		Ok(Self {
			inner: chorus::types::Sticker {
				id,
				guild_id,
				user: None,
				pack_id,
				name: name.to_string(),
				description,
				tags,
				asset: None,
				sticker_type,
				format_type: sticker_format_type,
				available: None,
				sort_value: None,
			},
			user_id,
		})
	}

	pub async fn populate_relations(&mut self, db: &PgPool) -> Result<(), Error> {
		if let Some(user_id) = self.user_id {
			self.user = User::get_by_id(db, user_id).await?.map(|user| user.to_inner());
		}
		Ok(())
	}

	pub async fn get_by_id(db: &PgPool, id: Snowflake) -> Result<Option<Self>, Error> {
		sqlx::query_as("SELECT * FROM stickers WHERE id = ?")
			.bind(id)
			.fetch_optional(db)
			.await
			.map_err(Error::Sqlx)
	}

	pub async fn get_by_guild(db: &PgPool, guild_id: Snowflake) -> Result<Vec<Self>, Error> {
		sqlx::query_as("SELECT * FROM stickers WHERE guild_id = ?")
			.bind(guild_id)
			.fetch_all(db)
			.await
			.map_err(Error::Sqlx)
	}

	pub async fn count_by_guild(db: &PgPool, guild_id: Snowflake) -> Result<i32, Error> {
		sqlx::query("SELECT COUNT(*) FROM stickers WHERE guild_id = ?")
			.bind(guild_id)
			.fetch_one(db)
			.await
			.map_err(Error::Sqlx)
			.map(|r| r.get::<i32, _>(0))
	}

	pub async fn save(&self, db: &PgPool) -> Result<(), Error> {
		sqlx::query("UPDATE stickers SET name = ?, description = ?, tags = ? WHERE id = ?")
			.bind(&self.name)
			.bind(&self.description)
			.bind(&self.tags)
			.bind(self.id)
			.execute(db)
			.await?;

		Ok(())
	}

	pub async fn delete(self, db: &PgPool) -> Result<(), Error> {
		sqlx::query("DELETE FROM stickers WHERE id = ?")
			.bind(self.id)
			.execute(db)
			.await
			.map_err(Error::Sqlx)
			.map(|_| ())
	}

	pub fn into_inner(self) -> chorus::types::Sticker {
		self.inner
	}
}
