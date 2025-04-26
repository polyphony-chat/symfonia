use std::fmt::Display;

use secrecy::{
	SecretBox,
	zeroize::{Zeroize, ZeroizeOnDrop},
};
use sqlx::{PgPool, postgres::PgConnectOptions};

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
