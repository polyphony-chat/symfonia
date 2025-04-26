// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

use chorus::types::{Snowflake, UserGuildSettingsUpdate};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use sqlx_pg_uint::{PgU16, PgU64};

use crate::{
	entities::{Guild, User},
	errors::{Error, GuildError, UserError},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, FromRow)]
pub struct GuildMember {
	#[serde(flatten)]
	#[sqlx(flatten)]
	inner: chorus::types::GuildMember,
	pub index: PgU64,
	pub id: Snowflake,
	pub guild_id: Snowflake,
	pub settings: sqlx::types::Json<UserGuildSettingsUpdate>,
	#[sqlx(skip)]
	pub user_data: User,
	pub last_message_id: Option<Snowflake>,
}

impl Deref for GuildMember {
	type Target = chorus::types::GuildMember;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for GuildMember {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl GuildMember {
	pub async fn create(db: &sqlx::PgPool, user: &User, guild: &Guild) -> Result<Self, Error> {
		// TODO: check if user is banned
		// TODO: Check max guild count

		if let Err(e) = GuildMember::get_by_id(db, user.id, guild.id).await {
			match e {
				Error::Guild(GuildError::MemberNotFound) => {
					// Continue adding user to guild
				}
				_ => return Err(e),
			}
		}

		let user = user.to_owned();
		let mut member = Self {
			index: 0.into(),
			id: user.id,
			guild_id: guild.id,
			settings: Default::default(),
			user_data: user.to_owned(),
			inner: chorus::types::GuildMember {
				user: None,
				nick: None,
				avatar: None, // TODO
				roles: vec![guild.id],
				joined_at: chrono::Utc::now(),
				..Default::default()
			},
			last_message_id: None,
		};

		let res = sqlx::query("INSERT INTO members (id, guild_id, joined_at, deaf, mute, pending, settings, bio) VALUES (?, ?, NOW(), 0, 0, 0, ?, ?) RETURNING members_index_seq")
            .bind(user.id)
            .bind(guild.id)
            .bind(sqlx::types::Json(UserGuildSettingsUpdate::default()))
            .bind(user.bio.clone().unwrap_or_default())
            .fetch_one(db)
            .await
            .map_err(Error::from)?;

		let index = PgU64::from_row(&res)?;

		member.index = index.clone();

		sqlx::query("INSERT INTO member_roles (index, role_id) VALUES (?,?)")
			.bind(index)
			.bind(guild.id)
			.execute(db)
			.await?;

		Ok(member)
	}

	pub async fn get_by_id(
		db: &sqlx::PgPool,
		id: Snowflake,
		guild_id: Snowflake,
	) -> Result<Option<Self>, Error> {
		let mut member: Self =
			sqlx::query_as("SELECT * FROM members WHERE id = ? AND guild_id = ?")
				.bind(id)
				.bind(guild_id)
				.fetch_optional(db)
				.await
				.map_err(Error::from)?
				.ok_or(Error::Guild(GuildError::MemberNotFound))?;

		// TODO: combine these queries with a JOIN

		let user = User::get_by_id(db, id).await?.ok_or(Error::User(UserError::InvalidUser))?;

		member.user_data = user;

		Ok(Some(member))
	}

	pub async fn get_by_guild_id(
		db: &sqlx::PgPool,
		guild_id: Snowflake,
		limit: u16,
		after: Option<Snowflake>,
	) -> Result<Vec<Self>, Error> {
		let limit = PgU16::from(limit);
		sqlx::query_as("SELECT * FROM members WHERE guild_id = ? WHERE id > IFNULL(?, 0) LIMIT ?")
			.bind(guild_id)
			.bind(limit)
			.bind(after)
			.fetch_all(db)
			.await
			.map_err(Error::from)
	}

	pub async fn get_by_role_id(
		db: &sqlx::PgPool,
		guild_id: Snowflake,
		role_id: Snowflake,
	) -> Result<Vec<Self>, Error> {
		sqlx::query_as("SELECT gm.* FROM members gm JOIN member_roles mr ON mr.index = gm.index WHERE mr.role_id = ? AND gm.guild_id = ?")
            .bind(role_id)
            .bind(guild_id)
            .fetch_all(db)
            .await
            .map_err(Error::from)
	}

	pub async fn get_by_user_id(db: &sqlx::PgPool, user_id: Snowflake) -> Result<Vec<Self>, Error> {
		sqlx::query_as("SELECT * FROM members WHERE id = ?")
			.bind(user_id)
			.fetch_all(db)
			.await
			.map_err(Error::from)
	}

	pub async fn search(
		db: &sqlx::PgPool,
		guild_id: Snowflake,
		query: &str,
		limit: u16,
	) -> Result<Vec<Self>, Error> {
		let limit = PgU16::from(limit);
		let mut members: Vec<Self> =
			sqlx::query_as("SELECT * FROM members WHERE guild_id = ? AND name LIKE ? LIMIT ?")
				.bind(guild_id)
				.bind(format!("%{}%", query))
				.bind(limit)
				.fetch_all(db)
				.await
				.map_err(Error::from)?;

		for member in members.iter_mut() {
			member.populate_relations(db).await?;
		}

		Ok(members)
	}

	pub async fn populate_relations(&mut self, db: &sqlx::PgPool) -> Result<(), Error> {
		// let guild = self.get_guild(db).await?;

		self.user_data = self.get_user(db).await?;
		self.user = Some(self.user_data.to_public_user());

		Ok(())
	}

	pub async fn count(db: &sqlx::PgPool) -> Result<i32, Error> {
		sqlx::query("SELECT COUNT(*) FROM members")
			.fetch_one(db)
			.await
			.map_err(Error::from)
			.map(|row| row.get::<i32, _>(0))
	}

	pub async fn count_by_role(db: &sqlx::PgPool, role_id: Snowflake) -> Result<i32, Error> {
		sqlx::query("SELECT COUNT(*) FROM member_roles WHERE role_id = ?")
			.bind(role_id)
			.fetch_one(db)
			.await
			.map_err(Error::from)
			.map(|row| row.get::<i32, _>(0))
	}

	pub async fn count_by_user_id(db: &sqlx::PgPool, user_id: Snowflake) -> Result<i32, Error> {
		sqlx::query("SELECT COUNT(*) FROM members WHERE id = ?")
			.bind(user_id)
			.fetch_one(db)
			.await
			.map(|r| r.get::<i32, _>(0))
			.map_err(Error::from)
	}

	pub async fn delete(self, db: &sqlx::PgPool) -> Result<(), Error> {
		sqlx::query("DELETE FROM members WHERE id =?")
			.bind(self.id)
			.execute(db)
			.await
			.map_err(Error::from)
			.map(|_| ())
	}

	pub async fn save(&self, db: &sqlx::PgPool) -> Result<(), Error> {
		sqlx::query("UPDATE members SET settings = ?, nick = ?, deaf = ?, mute = ?, pending = ?, last_message_id = ?, avatar = ?, flags = ?, permissions = ? WHERE id = ?") //banner = ?, bio = ?, theme_colors = ?,
            .bind(&self.settings)
            .bind(&self.nick)
            .bind(self.deaf)
            .bind(self.mute)
            .bind(self.pending)
            .bind(self.last_message_id)
            .bind(&self.avatar)
            // .bind(self.banner)
            // .bind(self.bio)
            // .bind(self.theme_colors)
            .bind(self.flags)
            .bind(&self.permissions)
            .bind(self.id)
            .execute(db)
            .await
            .map(|_| ())
            .map_err(Error::from)
	}

	// Start helper functions

	pub async fn get_guild(&self, db: &sqlx::PgPool) -> Result<Guild, Error> {
		Guild::get_by_id(db, self.guild_id)
			.await
			.and_then(|r| r.ok_or(Error::Guild(GuildError::InvalidGuild)))
	}

	pub async fn get_user(&self, db: &sqlx::PgPool) -> Result<User, Error> {
		User::get_by_id(db, self.id)
			.await
			.and_then(|r| r.ok_or(Error::User(UserError::InvalidUser)))
	}

	pub async fn add_role(&mut self, db: &sqlx::PgPool, role_id: Snowflake) -> Result<(), Error> {
		if self.roles.contains(&role_id) {
			return Ok(());
		}

		self.roles.push(role_id);
		sqlx::query("INSERT INTO member_roles (index, role_id) VALUES (?,?)")
			.bind(&self.index)
			.bind(role_id)
			.execute(db)
			.await?;

		Ok(())
	}

	pub async fn remove_role(
		&mut self,
		db: &sqlx::PgPool,
		role_id: Snowflake,
	) -> Result<(), Error> {
		if !self.roles.contains(&role_id) {
			return Ok(());
		}

		self.roles.retain(|r| r != &role_id);
		sqlx::query("DELETE FROM member_roles WHERE index =? AND role_id =?")
			.bind(&self.index)
			.bind(role_id)
			.execute(db)
			.await?;

		Ok(())
	}

	pub fn into_inner(self) -> chorus::types::GuildMember {
		self.inner
	}
}
