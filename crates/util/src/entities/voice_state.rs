// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

use chorus::types::Snowflake;
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{entities::Guild, errors::Error};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct VoiceState {
	#[sqlx(flatten)]
	inner: chorus::types::VoiceState,
	pub id: Snowflake,
}

impl Deref for VoiceState {
	type Target = chorus::types::VoiceState;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for VoiceState {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl VoiceState {
	// pub async fn create(
	//     guild_id: Snowflake,
	//     channel_id: Snowflake,
	//     user_id: Snowflake,
	//     session_id: String,
	//     deaf: bool,
	//     mute: bool,
	//     self_deaf: bool,
	// )

	pub async fn get_by_id(db: &PgPool, id: Snowflake) -> Result<Option<Self>, Error> {
		sqlx::query_as("SELECT * FROM voice_states WHERE id = ?")
			.bind(id)
			.fetch_optional(db)
			.await
			.map_err(Error::from)
	}

	pub async fn get_by_guild_and_channel(
		db: &PgPool,
		guild_id: Snowflake,
		channel_id: Option<Snowflake>,
		user_id: Snowflake,
	) -> Result<Option<Self>, Error> {
		sqlx::query_as(
			"SELECT * FROM voice_states WHERE guild_id = ? AND channel_id = ? AND user_id = ?",
		)
		.bind(guild_id)
		.bind(channel_id)
		.bind(user_id)
		.fetch_optional(db)
		.await
		.map_err(Error::from)
	}

	pub async fn populate_relations(&mut self, db: &PgPool) -> Result<(), Error> {
		if let Some(guild_id) = self.guild_id {
			let guild = Guild::get_by_id(db, guild_id).await?;
			if let Some(guild) = &guild {
				self.member = guild.get_member(db, self.user_id).await?.map(|m| m.into_inner());
			}
			self.guild = guild.map(|g| g.into_inner());
		}

		Ok(())
	}

	pub async fn save(&self, db: &PgPool) -> Result<(), Error> {
		sqlx::query(
			"UPDATE voice_states SET suppress = ?, request_to_speak_timestamp = ? WHERE id = ?",
		)
		.bind(self.suppress)
		.bind(self.request_to_speak_timestamp)
		.bind(self.id)
		.execute(db)
		.await
		.map(|_| ())
		.map_err(Error::from)
	}

	pub async fn delete(self, db: &PgPool) -> Result<(), Error> {
		sqlx::query("DELETE FROM voice_states WHERE id =?")
			.bind(self.id)
			.execute(db)
			.await
			.map(|_| ())
			.map_err(Error::from)
	}
}
