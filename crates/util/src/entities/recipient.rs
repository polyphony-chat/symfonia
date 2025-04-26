// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chorus::types::Snowflake;

use crate::{
	entities::{Channel, User},
	errors::Error,
};

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Recipient {
	pub id: Snowflake,
	#[sqlx(skip)]
	pub channel: Option<Channel>,
	pub channel_id: Snowflake,
	#[sqlx(skip)]
	pub user: Option<chorus::types::User>,
	pub user_id: Snowflake,
	pub closed: bool,
}

impl Recipient {
	pub async fn create(
		db: &sqlx::PgPool,
		channel_id: Snowflake,
		user_id: Snowflake,
	) -> Result<Self, Error> {
		let id = Snowflake::default();
		sqlx::query("INSERT INTO recipients (id, channel_id, user_id) VALUES (?,?,?)")
			.bind(id)
			.bind(channel_id)
			.bind(user_id)
			.execute(db)
			.await
			.map_err(Error::from)
			.map(|_| Recipient {
				id,
				channel_id,
				user_id,
				channel: None,
				user: None,
				closed: false,
			})
	}

	pub async fn populate_relations(&mut self, db: &sqlx::PgPool) -> Result<(), Error> {
		self.channel = Channel::get_by_id(db, self.channel_id).await?;
		self.user = User::get_by_id(db, self.user_id).await?.map(|u| u.to_inner());
		Ok(())
	}

	pub async fn get_by_id(db: &sqlx::PgPool, id: Snowflake) -> Result<Option<Self>, Error> {
		sqlx::query_as("SELECT * FROM recipients WHERE id = ?")
			.bind(id)
			.fetch_optional(db)
			.await
			.map_err(Error::from)
	}

	pub async fn get_by_channel_id(
		db: &sqlx::PgPool,
		channel_id: Snowflake,
	) -> Result<Vec<Self>, Error> {
		sqlx::query_as("SELECT * FROM recipients WHERE channel_id = ?")
			.bind(channel_id)
			.fetch_all(db)
			.await
			.map_err(Error::from)
	}

	pub async fn get_by_user_id(db: &sqlx::PgPool, user_id: Snowflake) -> Result<Vec<Self>, Error> {
		sqlx::query_as("SELECT * FROM recipients WHERE user_id = ?")
			.bind(user_id)
			.fetch_all(db)
			.await
			.map_err(Error::from)
	}

	pub async fn get_by_channel_and_user_id(
		db: &sqlx::PgPool,
		channel_id: Snowflake,
		user_id: Snowflake,
	) -> Result<Option<Self>, Error> {
		sqlx::query_as("SELECT * FROM recipients WHERE channel_id = ? AND user_id = ?")
			.bind(channel_id)
			.bind(user_id)
			.fetch_optional(db)
			.await
			.map_err(Error::from)
	}

	pub async fn delete(self, db: &sqlx::PgPool) -> Result<(), Error> {
		sqlx::query("DELETE FROM recipients WHERE id = ?")
			.bind(self.id)
			.execute(db)
			.await
			.map_err(Error::from)
			.map(|_| ())
	}
}
