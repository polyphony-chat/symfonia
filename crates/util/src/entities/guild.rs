// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

use chorus::types::{
	ChannelType, NSFWLevel, PermissionFlags, PremiumTier, Snowflake, SystemChannelFlags,
	WelcomeScreenObject,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, QueryBuilder, Row};
use sqlx_pg_uint::PgU16;

use super::*;
use crate::{
	SharedEventPublisherMap,
	entities::{Channel, Config, Emoji, GuildMember, GuildTemplate, Invite, Role, Sticker, User},
	errors::{Error, GuildError, UserError},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct Guild {
	#[sqlx(flatten)]
	#[serde(flatten)]
	inner: chorus::types::Guild,
	pub member_count: Option<i32>,
	pub presence_count: Option<i32>,
	pub unavailable: bool,
	pub parent: Option<String>,
	pub template_id: Option<Snowflake>,
	pub nsfw: bool,
	#[sqlx(skip)]
	#[serde(skip)]
	pub publisher: SharedEventPublisher,
}

impl Deref for Guild {
	type Target = chorus::types::Guild;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for Guild {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl Guild {
	pub async fn create(
		db: &PgPool,
		shared_event_publisher_map: SharedEventPublisherMap,
		cfg: &Config,
		name: &str,
		icon: Option<String>,
		owner_id: Snowflake,
		channels: &[chorus::types::Channel],
	) -> Result<Self, Error> {
		let mut guild = Self {
			inner: chorus::types::Guild {
				name: Some(name.to_string()),
				icon: Default::default(), // TODO: Handle guild Icon
				owner_id: Some(owner_id.to_owned()),
				preferred_locale: Some("en-US".to_string()),
				system_channel_flags: Some(
					SystemChannelFlags::SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATIONS,
				),
				welcome_screen: sqlx::types::Json(Some(WelcomeScreenObject {
					enabled: false,
					description: Some("Fill in your description".to_string()),
					welcome_channels: Vec::default(),
				})),
				afk_timeout: Some(cfg.defaults.guild.afk_timeout as i32),
				default_message_notifications: Some(
					cfg.defaults.guild.default_message_notifications,
				),
				explicit_content_filter: Some(cfg.defaults.guild.explicit_content_filter),
				features: cfg.guild.default_features.clone().into(),
				max_members: Some(cfg.limits.guild.max_members as i32),
				max_presences: Some(cfg.defaults.guild.max_presences as i32),
				max_video_channel_users: Some(cfg.defaults.guild.max_video_channel_users as i32),
				region: Some(cfg.regions.default.clone()),
				premium_tier: Some(PremiumTier::Tier3), // Maybe make this configurable?
				nsfw_level: Some(NSFWLevel::Default),
				..Default::default()
			},
			..Default::default()
		};
		shared_event_publisher_map.write().insert(guild.id, guild.publisher.clone());

		sqlx::query("INSERT INTO guilds (id, afk_timeout, default_message_notifications, explicit_content_filter, features, icon, max_members, max_presences, max_video_channel_users, name, owner_id, region, system_channel_flags, preferred_locale, welcome_screen, large, premium_tier, unavailable, widget_enabled, nsfw) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,0,?,0,0,?)")
            .bind(guild.id)
            .bind(guild.afk_timeout)
            .bind(guild.default_message_notifications)
            .bind(guild.explicit_content_filter)
            .bind(&guild.features)
            .bind(&guild.icon)
            .bind(guild.max_members)
            .bind(guild.max_presences)
            .bind(guild.max_video_channel_users)
            .bind(&guild.name)
            .bind(guild.owner_id)
            .bind(&guild.region)
            .bind(guild.system_channel_flags)
            .bind(&guild.preferred_locale)
            .bind(&guild.welcome_screen)
            .bind(guild.premium_tier)
            .bind(true) // TODO: Do this better guild.nsfw_level
            .execute(db)
            .await?;
		log::debug!(target: "symfonia::guilds", "Created guild {:?} with id {}", name, guild.id);

		let everyone = Role::create(
			db,
			shared_event_publisher_map,
			Some(guild.id),
			guild.id,
			"@everyone",
			0.,
			false,
			true,
			false,
			PermissionFlags::from_bits(2251804225).unwrap(), /* 559623605571137? make it load
			                                                  * from config? */
			0,
			None,
			None,
		)
		.await?;

		let user = User::get_by_id(db, owner_id).await?.unwrap();

		user.add_to_guild(db, guild.id).await?;
		guild.owner = Some(true);

		guild.roles = vec![everyone.into_inner()];

		let channels = if channels.is_empty() {
			vec![
				Channel::create(
					db,
					ChannelType::GuildText,
					Some("general".to_string()),
					false,
					Some(guild.id.to_owned()),
					None,
					false,
					false,
					false,
					false,
					vec![],
				)
				.await?,
			]
		} else {
			let mut new_channels = Vec::with_capacity(channels.len());
			for channel in channels {
				new_channels.push(
					Channel::create(
						db,
						channel.channel_type,
						channel.name.to_owned(),
						channel.nsfw.unwrap_or(false),
						Some(guild.id),
						channel.parent_id,
						false,
						false,
						false,
						false,
						channel.permission_overwrites.clone().unwrap_or_default().0,
					)
					.await?,
				);
			}
			new_channels
		};

		guild.channels = channels.into_iter().map(|c| c.into_inner()).collect();

		Ok(guild)
	}

	pub async fn create_from_template(
		db: &PgPool,
		cfg: &Config,
		shared_event_publisher_map: SharedEventPublisherMap,
		owner_id: Snowflake,
		template: &GuildTemplate,
		name: &str,
	) -> Result<Self, Error> {
		let Some(g) = template.serialized_source_guild.first() else {
			return Err(Error::Guild(GuildError::NoSourceGuild));
		};

		Self::create(db, shared_event_publisher_map, cfg, name, None, owner_id, &g.channels).await
	}

	pub async fn get_by_id(db: &PgPool, id: Snowflake) -> Result<Option<Self>, Error> {
		sqlx::query_as("SELECT * FROM guilds WHERE id = ?")
			.bind(id)
			.fetch_optional(db)
			.await
			.map_err(Error::Sqlx)
	}

	// Helper functions start
	pub async fn get_member(
		&self,
		db: &PgPool,
		user_id: Snowflake,
	) -> Result<Option<GuildMember>, Error> {
		GuildMember::get_by_id(db, user_id, self.id).await
	}

	pub async fn get_members_by_role(
		&self,
		db: &PgPool,
		role_id: Snowflake,
	) -> Result<Vec<GuildMember>, Error> {
		GuildMember::get_by_role_id(db, role_id, self.id).await
	}

	pub async fn has_member(&self, db: &PgPool, user_id: Snowflake) -> Result<bool, Error> {
		sqlx::query_as("SELECT * FROM guild_members WHERE guild_id = ? AND user_id =?")
			.bind(self.id)
			.bind(user_id)
			.fetch_optional(db)
			.await
			.map_err(Error::Sqlx)
			.map(|r: Option<GuildMember>| r.is_some())
	}

	pub async fn get_role(&self, db: &PgPool, id: Snowflake) -> Result<Option<Role>, Error> {
		Role::get_by_id(db, id).await.map(|x| x.filter(|y| y.guild_id == self.id))
	}

	pub async fn get_invites(&self, db: &PgPool) -> Result<Vec<Invite>, Error> {
		Invite::get_by_guild(db, self.id).await
	}

	pub async fn get_emoji(&self, db: &PgPool, id: Snowflake) -> Result<Option<Emoji>, Error> {
		Emoji::get_by_id(db, id)
			.await // We only want emojis from this guild
			.map(|x| x.filter(|y| y.guild_id == self.id))
	}

	pub async fn get_emojis(&self, db: &PgPool) -> Result<Vec<Emoji>, Error> {
		Emoji::get_by_guild(db, self.id).await
	}

	pub async fn get_stickers(&self, db: &PgPool) -> Result<Vec<Sticker>, Error> {
		Sticker::get_by_guild(db, self.id).await
	}

	pub async fn get_roles(&self, db: &PgPool) -> Result<Vec<Role>, Error> {
		Role::get_by_guild(db, self.id).await
	}

	pub async fn count_roles(&self, db: &PgPool) -> Result<i32, Error> {
		Role::count_by_guild(db, self.id).await
	}

	pub async fn count(db: &PgPool) -> Result<i32, Error> {
		sqlx::query("SELECT COUNT(*) FROM guilds")
			.fetch_one(db)
			.await
			.map_err(Error::Sqlx)
			.map(|r| r.get::<i32, _>(0))
	}

	pub async fn populate_relations(&mut self, db: &PgPool) -> Result<(), Error> {
		self.emojis = self.get_emojis(db).await?.into_iter().map(|e| e.into_inner()).collect();

		self.roles = self.get_roles(db).await?.into_iter().map(|r| r.into_inner()).collect();

		self.stickers = self.get_stickers(db).await?.into_iter().map(|s| s.into_inner()).collect();
		Ok(())
	}

	/// Shorthand for `GuildMember::create()`
	pub async fn add_member(&self, db: &PgPool, user_id: Snowflake) -> Result<(), Error> {
		let user =
			User::get_by_id(db, user_id).await?.ok_or(Error::User(UserError::InvalidUser))?;

		let member = GuildMember::create(db, &user, self).await?;

		Ok(())
	}

	pub async fn calculate_inactive_members(
		&self,
		db: &PgPool,
		days: u8,
		roles: Vec<Snowflake>,
		highest_role: PgU16,
	) -> Result<Vec<GuildMember>, Error> {
		let now = chrono::Utc::now().naive_utc();
		let cutoff = now - chrono::Duration::days(days as i64);
		let min_snowflake = Snowflake::from(cutoff.and_utc().timestamp() as u64);

		if roles.is_empty() {
			sqlx::query_as("SELECT gm.* FROM guild_members gm JOIN member_roles mr ON gm.index = mr.index JOIN roles r ON r.id = mr.role_id WHERE gm.guild_id = ? AND (gm.last_message_id < ? OR gm.last_message_id IS NULL) AND r.position < ?")
                .bind(self.id)
                .bind(min_snowflake)
                .bind(highest_role)
                .fetch_all(db)
                .await
                .map_err(Error::Sqlx)
		} else {
			let mut builder = QueryBuilder::new(
				"SELECT gm.* FROM members gm JOIN member_roles mr ON gm.index = mr.index JOIN roles r ON r.id = mr.role_id WHERE gm.guild_id = ? AND (gm.last_message_id < ? OR gm.last_message_id IS NULL) AND r.position < ? AND mr.role_id IN (",
			);

			let mut separated = builder.separated(", ");
			for role in &roles {
				separated.push_bind(role);
			}
			separated.push_unseparated(") ");

			let query = builder.build();
			Ok(query
				.bind(self.id)
				.bind(min_snowflake)
				.bind(highest_role)
				.fetch_all(db)
				.await?
				.iter_mut()
				.flat_map(|row| GuildMember::from_row(row))
				.collect())
		}
	}

	pub async fn save(&self, db: &PgPool) -> Result<(), Error> {
		sqlx::query("UPDATE guilds SET afk_timeout =?, default_message_notifications =?, explicit_content_filter =?, features =?, icon =?, max_members =?, max_presences =?, max_video_channel_users =?, name =?, owner_id =?, region =?, system_channel_flags =?, preferred_locale =?, welcome_screen =?, large =?, premium_tier =?, unavailable =?, widget_enabled =?, nsfw =?, public_updates_channel_id =?, rules_channel_id =? WHERE id =?")
            .bind(self.afk_timeout)
            .bind(self.default_message_notifications)
            .bind(self.explicit_content_filter)
            .bind(&self.features)
            .bind(&self.icon)
            .bind(self.max_members)
            .bind(self.max_presences)
            .bind(self.max_video_channel_users)
            .bind(&self.name)
            .bind(self.owner_id)
            .bind(&self.region)
            .bind(self.system_channel_flags)
            .bind(&self.preferred_locale)
            .bind(&self.welcome_screen)
            .bind(self.premium_tier)
            .bind(self.unavailable)
            .bind(self.widget_enabled)
            .bind(self.nsfw)
            .bind(self.public_updates_channel_id)
            .bind(self.rules_channel_id)
            .bind(self.id)
            .execute(db)
            .await
            .map(|_| ())
            .map_err(Error::Sqlx)
	}

	pub async fn delete(self, db: &PgPool) -> Result<(), Error> {
		sqlx::query("DELETE FROM guilds WHERE id =?")
			.bind(self.id)
			.execute(db)
			.await
			.map_err(Error::Sqlx)
			.map(|_| ())
	}

	pub async fn search_members(
		&self,
		db: &sqlx::PgPool,
		query: &str,
		limit: u16,
	) -> Result<Vec<GuildMember>, Error> {
		GuildMember::search(db, self.id, query, limit).await
	}

	pub fn into_inner(self) -> chorus::types::Guild {
		self.inner
	}

	pub fn to_inner(&self) -> &chorus::types::Guild {
		&self.inner
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GuildBan {
	#[sqlx(flatten)]
	inner: chorus::types::GuildBan,
	pub id: Snowflake,
	pub executor_id: Snowflake,
	pub guild_id: Snowflake,
	pub user_id: Snowflake,
	pub ip: String,
}

impl Deref for GuildBan {
	type Target = chorus::types::GuildBan;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for GuildBan {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl GuildBan {
	pub async fn create(
		db: &PgPool,
		guild_id: Snowflake,
		user_id: Snowflake,
		executing_user_id: Snowflake,
		reason: impl Into<Option<String>>,
	) -> Result<Self, Error> {
		// TODO: Remove the user from the Guild
		let ban_id = Snowflake::default();
		let reason = reason.into();
		let user = GuildMember::get_by_id(db, user_id, guild_id)
			.await?
			.ok_or(Error::Guild(GuildError::MemberNotFound))?;

		sqlx::query("INSERT INTO guild_bans (id, guild_id, user_id, executor_id, reason, ip) VALUES (?,?,?,?,?,'127.0.0.1')") // TODO: Do something to get the users IP
            .bind(ban_id)
            .bind(guild_id)
            .bind(user_id)
            .bind(executing_user_id)
            .bind(&reason)
            .execute(db)
            .await
            .map_err(Error::Sqlx)?;

		Ok(Self {
			inner: chorus::types::GuildBan { reason, user: user.user_data.to_public_user() },
			guild_id,
			user_id,
			executor_id: executing_user_id,
			id: ban_id,
			ip: "127.0.0.1".to_string(),
		})
	}

	pub async fn builk_create(
		db: &PgPool,
		guild_id: Snowflake,
		user_ids: Vec<Snowflake>,
		executing_user_id: Snowflake,
		reason: impl Into<Option<String>>,
	) -> Result<Vec<GuildBan>, Error> {
		let mut query_builder = QueryBuilder::new(
			"INSERT INTO guild_bans (id, guild_id, user_id, executor_id, reason, ip) VALUES ",
		);
		let mut rows = user_ids
			.into_iter()
			.map(|user_id| GuildBan {
				inner: chorus::types::GuildBan { user: Default::default(), reason: None },
				id: Snowflake::default(),
				executor_id: executing_user_id,
				guild_id,
				user_id,
				ip: "127.0.0.1".to_string(), // TODO: Somehow get the users IP
			})
			.collect::<Vec<_>>();

		let reason = reason.into();

		query_builder.push_values(rows.iter(), |mut b, user| {
			b.push_bind(user.id)
				.push_bind(user.guild_id)
				.push_bind(user.user_id)
				.push_bind(user.executor_id)
				.push_bind(&reason)
				.push_bind("127.0.0.1");
		});

		let query = query_builder.build();

		query.execute(db).await?;

		for row in rows.iter_mut() {
			row.user = User::get_by_id(db, row.user_id)
				.await?
				.ok_or(Error::User(UserError::InvalidUser))?
				.to_public_user();
		}

		Ok(rows)
	}

	pub async fn populate_relations(&mut self, db: &PgPool) -> Result<(), Error> {
		let user =
			User::get_by_id(db, self.user_id).await?.ok_or(Error::User(UserError::InvalidUser))?;
		self.user = user.to_public_user();
		Ok(())
	}

	pub async fn get_by_id(db: &PgPool, id: Snowflake) -> Result<Option<GuildBan>, Error> {
		sqlx::query_as("SELECT * FROM guild_bans WHERE id = ?")
			.bind(id)
			.fetch_optional(db)
			.await
			.map_err(Error::Sqlx)
	}

	pub async fn get_by_guild(
		db: &PgPool,
		guild_id: Snowflake,
		before: Option<Snowflake>,
		after: Option<Snowflake>,
		limit: Option<u16>,
	) -> Result<Vec<GuildBan>, Error> {
		let limit = limit.map(PgU16::from);
		sqlx::query_as("SELECT * FROM bans WHERE (user_id < ? OR ? IS NULL) AND (user_id > ? OR ? IS NULL) AND guild_id = ? LIMIT IFNULL(?, 1000)")
            .bind(before)
            .bind(before)
            .bind(after)
            .bind(after)
            .bind(guild_id)
            .bind(limit)
            .fetch_all(db)
            .await
            .map_err(Error::Sqlx)
	}

	pub async fn get_by_user(
		db: &PgPool,
		guild_id: Snowflake,
		user_id: Snowflake,
	) -> Result<Option<Self>, Error> {
		sqlx::query_as("SELECT * FROM guild_bans WHERE user_id = ? AND guild_id = ?")
			.bind(user_id)
			.bind(guild_id)
			.fetch_optional(db)
			.await
			.map_err(Error::Sqlx)
	}

	pub async fn find_by_username(
		db: &PgPool,
		guild_id: Snowflake,
		search_term: &str,
		limit: PgU16,
	) -> Result<Vec<Self>, Error> {
		sqlx::query_as("SELECT b.* FROM bans b JOIN members m ON b.user_id = m.id AND b.guild_id = m.guild_id JOIN users u ON b.user_id = u.id WHERE u.username LIKE ? AND b.guild_id = ? LIMIT ?")
            .bind(format!("%{}%", search_term))
            .bind(guild_id)
            .bind(limit)
            .fetch_all(db)
            .await
            .map_err(Error::Sqlx)
	}

	pub async fn delete(self, db: &PgPool) -> Result<(), Error> {
		sqlx::query("DELETE FROM guild_bans WHERE id = ?")
			.bind(self.id)
			.execute(db)
			.await
			.map_err(Error::Sqlx)?;
		Ok(())
	}

	// Start helper functions

	pub fn into_inner(self) -> chorus::types::GuildBan {
		self.inner
	}
}
