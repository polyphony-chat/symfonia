// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

use chorus::types::{Snowflake, WebhookType};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};

use crate::errors::Error;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Webhook {
	#[sqlx(flatten)]
	inner: chorus::types::Webhook,
	pub source_guild_id: Option<Snowflake>,
	pub user_id: Snowflake,
}

impl Deref for Webhook {
	type Target = chorus::types::Webhook;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for Webhook {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl Webhook {
	pub async fn create(
		db: &PgPool,
		name: &str,
		guild_id: Snowflake,
		channel_id: Snowflake,
		user_id: Snowflake,
		avatar: Option<String>,
		webhook_type: WebhookType,
		source_guild_id: Option<Snowflake>,
		application_id: Option<Snowflake>,
	) -> Result<Self, Error> {
		let token_data = [0u8; 24];

		let webhook = Self {
			inner: chorus::types::Webhook {
				id: Default::default(),
				token: hex::encode(token_data),
				guild_id,
				channel_id,
				name: name.to_string(),
				avatar: avatar.unwrap_or_default(), // TODO: Some default avatar data?
				webhook_type,
				application_id,
				user: None,         // User::get_by_id(db, user_id).await?.map(Shared),
				source_guild: None, // Guild::get_by_id(db, source_guild_id).await?.map(Shared)
				url: None,
			},
			source_guild_id,
			user_id,
		};

		sqlx::query("INSERT INTO webhooks (id, token, guild_id, channel_id, name, avatar, webhook_type, application_id, user_id, source_guild_id) VALUES (?,?,?,?,?,?,?,?,?,?)")
            .bind(webhook.id)
            .bind(&webhook.token)
            .bind(webhook.guild_id)
            .bind(webhook.channel_id)
            .bind(&webhook.name)
            .bind(&webhook.avatar)
            .bind(webhook.webhook_type)
            .bind(webhook.application_id)
            .bind(webhook.user_id)
            .bind(webhook.source_guild_id)
            .execute(db)
            .await?;

		Ok(webhook)
	}

	pub async fn get_by_id(db: &PgPool, id: Snowflake) -> Result<Option<Self>, Error> {
		sqlx::query_as("SELECT * FROM webhooks WHERE id =?")
			.bind(id)
			.fetch_optional(db)
			.await
			.map_err(Error::Sqlx)
	}

	pub async fn get_by_channel_id(db: &PgPool, channel_id: Snowflake) -> Result<Vec<Self>, Error> {
		sqlx::query_as("SELECT * FROM webhooks WHERE channel_id =?")
			.bind(channel_id)
			.fetch_all(db)
			.await
			.map_err(Error::Sqlx)
	}

	pub async fn count_by_channel(db: &PgPool, channel_id: Snowflake) -> Result<i32, Error> {
		sqlx::query("SELECT COUNT(*) FROM webhooks WHERE channel_id = ?")
			.bind(channel_id)
			.fetch_one(db)
			.await
			.map_err(Error::Sqlx)
			.map(|row| row.get::<i32, _>(0))
	}
}
