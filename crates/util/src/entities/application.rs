// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
	ops::{Deref, DerefMut},
	sync::Arc,
};

use chorus::types::{ApplicationFlags, Snowflake};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::{Config, user::User, *};
use crate::errors::Error;

#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct Application {
	#[sqlx(flatten)]
	inner: chorus::types::Application,
	pub owner_id: Snowflake,
	pub bot_user_id: Option<Snowflake>,
	pub team_id: Option<Snowflake>,
	#[sqlx(skip)]
	#[serde(skip)]
	pub publisher: SharedEventPublisher,
}

impl Deref for Application {
	type Target = chorus::types::Application;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for Application {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl Application {
	pub async fn create(
		db: &PgPool,
		cfg: &Config,
		name: &str,
		summary: &str,
		owner_id: &Snowflake,
		verify_key: &str,
		flags: ApplicationFlags,
		create_bot_user: bool,
	) -> Result<Self, Error> {
		let bot_user_id = if create_bot_user {
			let bot_user = User::create(db, cfg, name, None, None, None, None, true).await?;

			Some(bot_user.id.to_owned())
		} else {
			None
		};

		let application = Self {
			inner: chorus::types::Application {
				name: name.to_string(),
				summary: Some(summary.to_string()),
				verify_key: verify_key.to_string(),
				flags,
				..Default::default()
			},
			owner_id: owner_id.to_owned(),
			bot_user_id,
			team_id: None,
			publisher: Arc::new(RwLock::new(pubserve::Publisher::new())),
		};

		let _res = sqlx::query("INSERT INTO applications (id, name, summary, hook, bot_public, verify_key, owner_id, flags, integration_public, discoverability_state, discovery_eligibility_flags) VALUES (?, ?, ?, true, true, ?, ?, ?, true, 1, 2240)")
            .bind(application.id)
            .bind(name)
            .bind(summary)
            .bind(verify_key)
            .bind(owner_id)
            .bind(flags)
            .execute(db)
            .await?;

		Ok(application)
	}

	pub async fn get_by_id(db: &PgPool, id: &Snowflake) -> Result<Option<Self>, Error> {
		sqlx::query_as("SELECT * FROM applications WHERE id = ?")
			.bind(id)
			.fetch_optional(db)
			.await
			.map_err(Error::Sqlx)
	}

	pub async fn get_by_owner(db: &PgPool, owner_id: &Snowflake) -> Result<Vec<Self>, Error> {
		sqlx::query_as("SELECT * FROM applications WHERE owner_id = ?")
			.bind(owner_id)
			.fetch_all(db)
			.await
			.map_err(Error::Sqlx)
	}

	pub async fn get_owner(&self, db: &PgPool) -> Result<User, Error> {
		let u = User::get_by_id(db, self.owner_id).await?.unwrap(); // Unwrap the option since this should absolutely never fail
		Ok(u)
	}

	pub fn public_json(&self) -> String {
		serde_json::to_string(&self.inner).unwrap()
	}
}
