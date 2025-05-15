// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::Display;

use secrecy::{
	SecretBox,
	zeroize::{Zeroize, ZeroizeOnDrop},
};
use sqlx::{PgPool, Row, postgres::PgConnectOptions};

mod seed_config;
pub use seed_config::*;

use crate::errors::Error;

pub struct Connection {
	pool: PgPool,
}

impl Connection {
	pub async fn new(options: PgConnectOptions) -> Result<Self, sqlx::Error> {
		let pool = PgPool::connect_with(options).await?;
		Ok(Self { pool })
	}

	pub fn pool(&self) -> &PgPool {
		&self.pool
	}

	pub async fn create_user(
		name: Username,
		can_create_dbs: bool,
		can_create_users: bool,
		group_names: Vec<Groupname>,
		password: SecretBox<Password>,
		encrypt_password: bool,
		valid_until: Option<u64>,
	) -> Result<(), sqlx::Error> {
		todo!()
	}

	pub async fn check_fresh_db(&self) -> Result<bool, Error> {
		let res = sqlx::query("SELECT COUNT(*) FROM config").fetch_one(self.pool()).await?;
		let c: i64 = res.get(0);

		Ok(c == 0)
	}
}

pub struct Username {
	name: String,
}

impl Username {
	pub fn new(name: &str) -> Result<Self, Box<dyn Display>> {
		todo!()
	}
}

pub struct Groupname {
	name: String,
}

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Password {
	password: String,
}
