// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

use chorus::types::{CreateChannelInviteSchema, InviteType, Snowflake};
use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx_pg_uint::{PgU8, PgU32};

use crate::{
	entities::{Channel, Guild, User},
	errors::{ChannelError, Error, GuildError},
};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Invite {
	#[serde(flatten)]
	#[sqlx(flatten)]
	inner: chorus::types::Invite,
	#[serde(skip)]
	pub channel_id: Option<Snowflake>,
	#[serde(skip)]
	pub inviter_id: Option<Snowflake>,
	#[serde(skip)]
	pub target_user_id: Option<Snowflake>,
	pub vanity_url: Option<bool>,
}

impl Deref for Invite {
	type Target = chorus::types::Invite;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for Invite {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl Invite {
	pub async fn create(
		db: &PgPool,
		data: CreateChannelInviteSchema,
		channel_id: Option<Snowflake>,
		inviter_id: Option<Snowflake>,
		invite_type: InviteType,
	) -> Result<Self, Error> {
		let random_code = String::from_utf8(
			rand::thread_rng()
				.sample_iter(&rand::distributions::Alphanumeric)
				.take(8) // TODO: Make this configurable?
				.collect::<Vec<u8>>(),
		)?;

		let mut guild_id = None;
		if let Some(channel_id) = channel_id {
			let channel = Channel::get_by_id(db, channel_id)
				.await?
				.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

			guild_id = channel.inner.guild_id;
		}

		let expires_at = data.max_age.map(|age| Utc::now() + chrono::Duration::seconds(age as i64));

		let invite = Self {
			inner: chorus::types::Invite {
				approximate_member_count: None,
				approximate_presence_count: None,
				channel: None,
				code: random_code.to_owned(),
				created_at: Some(Utc::now()),
				expires_at,
				flags: data.flags,
				guild: None,
				guild_id,
				guild_scheduled_event: None,
				invite_type: Some(invite_type),
				inviter: None,
				max_age: data.max_age.map(|max_age| max_age.into()),
				max_uses: data.max_uses.map(|max_uses| max_uses.into()),
				stage_instance: None,
				target_application: None,
				target_type: data.target_type,
				target_user: None,
				temporary: data.temporary,
				uses: None,
			},
			channel_id,
			inviter_id,
			vanity_url: None,
			target_user_id: data.target_user_id,
		};

		/*
		code, type, temporary, uses, max_uses, max_age, created_at, expires_at, guild_id, channel_id, inviter_id, target_user_id, target_user_type, vanity_url, flags
		 */

		sqlx::query("INSERT INTO invites (code, type, temporary, uses, max_uses, max_age, created_at, expires_at, guild_id, channel_id, inviter_id, target_user_id, target_user_type, vanity_url, flags) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(random_code)
            .bind(invite.invite_type)
            .bind(invite.temporary)
            .bind(invite.uses.clone().unwrap_or(0.into()))
            .bind(invite.max_uses.clone())
            .bind(invite.max_age.clone())
            .bind(invite.created_at)
            .bind(invite.expires_at)
            .bind(invite.guild_id)
            .bind(invite.channel_id)
            .bind(invite.inviter_id)
            .bind(invite.target_user_id)
            .bind(invite.target_type)
            .bind(invite.vanity_url)
            .bind(invite.flags)
            .execute(db)
            .await?;

		Ok(invite)
	}

	pub async fn create_vanity(
		db: &PgPool,
		guild_id: Snowflake,
		code: &str,
	) -> Result<Self, Error> {
		let invite = Self {
			inner: chorus::types::Invite {
				code: code.to_string(),
				created_at: Some(Utc::now()),
				guild_id: Some(guild_id),
				invite_type: Some(InviteType::Guild),
				..Default::default()
			},
			channel_id: None,
			inviter_id: None,
			target_user_id: None,
			vanity_url: Some(true),
		};

		sqlx::query("INSERT INTO invites (code, type, temporary, uses, max_uses, max_age, created_at, expires_at, guild_id, channel_id, inviter_id, target_user_id, target_user_type, vanity_url, flags) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(code)
            .bind(invite.invite_type)
            .bind(invite.temporary)
            .bind(invite.uses.clone().unwrap_or(PgU32::from(0)))
            .bind(invite.max_uses.clone())
            .bind(invite.max_age.clone())
            .bind(invite.created_at)
            .bind(invite.expires_at)
            .bind(invite.guild_id)
            .bind(invite.channel_id)
            .bind(invite.inviter_id)
            .bind(invite.target_user_id)
            .bind(invite.target_type)
            .bind(invite.vanity_url)
            .bind(invite.flags)
            .execute(db)
            .await?;

		Ok(invite)
	}

	pub async fn get_by_code(db: &PgPool, code: &str) -> Result<Option<Self>, Error> {
		let invite: Option<Self> = sqlx::query_as("SELECT * FROM invites WHERE code = ?")
			.bind(code)
			.fetch_optional(db)
			.await
			.map_err(Error::Sqlx)?;

		Ok(invite)
	}

	pub async fn get_by_guild(db: &PgPool, guild_id: Snowflake) -> Result<Vec<Self>, Error> {
		let guild =
			Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

		let mut invites = sqlx::query_as("SELECT * FROM invites WHERE guild_id = ?")
			.bind(guild_id)
			.fetch_all(db)
			.await
			.map_err(Error::Sqlx)?;

		invites
			.iter_mut()
			.for_each(|invite: &mut Invite| invite.guild = Some(guild.clone().into_inner().into()));

		Ok(invites)
	}

	pub async fn get_by_guild_vanity(
		db: &PgPool,
		guild_id: Snowflake,
	) -> Result<Option<Self>, Error> {
		sqlx::query_as("SELECT * FROM invites WHERE guild_id = ? AND vanity_url = 1")
			.bind(guild_id)
			.fetch_optional(db)
			.await
			.map_err(Error::Sqlx)
	}

	pub async fn get_by_channel(db: &PgPool, channel_id: Snowflake) -> Result<Vec<Self>, Error> {
		sqlx::query_as("SELECT * FROM invites WHERE channel_id = ?")
			.bind(channel_id)
			.fetch_all(db)
			.await
			.map_err(Error::Sqlx)
	}

	pub async fn delete(&self, db: &PgPool) -> Result<(), Error> {
		sqlx::query("DELETE FROM invites WHERE code = ?")
			.bind(&self.code)
			.execute(db)
			.await
			.map(|_| ())
			.map_err(Error::Sqlx)
	}

	pub async fn join(&mut self, db: &PgPool, user: &User) -> Result<(), Error> {
		if let Some(invite_type) = self.invite_type {
			match invite_type {
				InviteType::Guild => {
					// TODO: Track what invite code a user used?
					user.add_to_guild(
						db,
						self.guild_id.ok_or(Error::Guild(GuildError::InvalidGuild))?,
					)
					.await?;
					self.increase_uses(db).await?;
				}
				InviteType::GroupDm => todo!(),
				InviteType::Friend => todo!(),
			}
		} else {
			// TODO: Some form of handling for invites without an invite type?
			// maybe just default to guild?
		}

		let max_uses = self.max_uses.clone().unwrap_or(PgU8::from(0)).to_uint() as u32;
		if self.uses.clone().unwrap_or(PgU32::from(0)) > PgU32::from(max_uses) && max_uses > 0 {
			self.delete(db).await?;
		}

		Ok(())
	}

	pub async fn increase_uses(&mut self, db: &PgPool) -> Result<(), Error> {
		self.uses = self.uses.as_mut().map(|uses| PgU32::from(uses.to_uint() + 1));
		sqlx::query("UPDATE invites SET uses = ? WHERE code = ?")
			.bind(&self.uses)
			.bind(&self.code)
			.execute(db)
			.await
			.map(|_| ())
			.map_err(Error::Sqlx)
	}

	pub async fn populate_relations(&mut self, db: &PgPool) -> Result<(), Error> {
		// if let Some(guild_id) = self.guild_id {
		//     self.guild = Guild::get_by_id(db, guild_id).await?.map(|guild|
		// GuildInvite::fr); }

		if let Some(target_user_id) = self.target_user_id {
			self.target_user =
				User::get_by_id(db, target_user_id).await?.map(|user| user.to_inner());
		}

		Ok(())
	}

	pub async fn set_code(&mut self, db: &PgPool, code: &str) -> Result<(), Error> {
		sqlx::query("UPDATE invites SET code = ? WHERE code = ?")
			.bind(code)
			.bind(&self.code)
			.execute(db)
			.await?;

		self.code = code.to_string();
		Ok(())
	}

	pub async fn save(&self, db: &PgPool) -> Result<(), Error> {
		sqlx::query("UPDATE invites SET type = ?, temporary = ?, uses = ?, max_uses = ?, max_age = ?, created_at = ?, expires_at = ?, guild_id = ?, channel_id = ?, inviter_id = ?, target_user_id = ?, target_user_type = ?, vanity_url = ?, flags = ? WHERE code = ?")
            .bind(&self.code)
            .bind(self.invite_type)
            .bind(self.temporary)
            .bind(self.uses.as_ref().unwrap_or(&0.into()))
            .bind(&self.max_uses)
            .bind(&self.max_age)
            .bind(self.created_at)
            .bind(self.expires_at)
            .bind(self.guild_id)
            .bind(self.channel_id)
            .bind(self.inviter_id)
            .bind(&self.code)
            .execute(db)
            .await
            .map(|_| ())
            .map_err(Error::Sqlx)
	}

	pub fn into_inner(self) -> chorus::types::Invite {
		self.inner
	}
}
