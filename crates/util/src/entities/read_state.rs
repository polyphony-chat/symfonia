// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chorus::types::Snowflake;
use sqlx::PgPool;

use crate::{
	entities::{Channel, User},
	errors::Error,
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ReadState {
	pub channel_id: Snowflake,
	#[sqlx(skip)]
	pub channel: Option<Channel>,
	pub user_id: Snowflake,
	#[sqlx(skip)]
	pub user: Option<User>,
	pub last_message_id: Option<Snowflake>,
	pub public_ack: Option<String>,
	pub notifications_cursor: Option<Snowflake>,
	pub last_pin_timestamp: Option<chrono::DateTime<chrono::Utc>>,
	pub mention_count: Option<i32>,
	#[sqlx(skip)]
	pub manual: bool,
}

impl ReadState {
	pub async fn create(
		db: &PgPool,
		channel_id: Snowflake,
		user_id: Snowflake,
		message_id: Option<Snowflake>,
	) -> Result<Self, Error> {
		sqlx::query(
			"INSERT INTO read_states (channel_id, user_id, last_message_id) VALUES (?,?,?)",
		)
		.bind(channel_id)
		.bind(user_id)
		.bind(message_id)
		.execute(db)
		.await?;

		Ok(Self {
			channel_id,
			channel: None,
			user_id,
			user: None,
			last_message_id: message_id,
			public_ack: None,
			notifications_cursor: None,
			last_pin_timestamp: None,
			mention_count: None,
			manual: false,
		})
	}
	pub async fn get_by_user_and_channel(
		db: &PgPool,
		channel_id: Snowflake,
		user_id: Snowflake,
	) -> Result<Option<Self>, Error> {
		sqlx::query_as("SELECT * FROM read_states WHERE channel_id = ? AND user_id =?")
			.bind(channel_id)
			.bind(user_id)
			.fetch_optional(db)
			.await
			.map_err(Error::Sqlx)
	}

	pub async fn populate_relations(&mut self, db: &PgPool) -> Result<(), Error> {
		self.user = User::get_by_id(db, self.user_id).await?;
		self.channel = Channel::get_by_id(db, self.channel_id).await?;
		Ok(())
	}

	pub async fn save(&self, db: &PgPool) -> Result<(), Error> {
		sqlx::query("INSERT INTO read_states (channel_id, user_id, last_message_id, public_ack, notifications_cursor, last_pin_timestamp, mention_count, manual) VALUES (?,?,?,?,?,?,?,?)")
           .bind(self.channel_id)
           .bind(self.user_id)
           .bind(self.last_message_id)
           .bind(&self.public_ack)
           .bind(self.notifications_cursor)
           .bind(self.last_pin_timestamp)
           .bind(self.mention_count)
           .bind(self.manual)
           .execute(db)
           .await
           .map_err(Error::Sqlx)
            .map(|_| ())
	}
}
