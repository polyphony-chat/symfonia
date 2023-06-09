pub mod entities;

use crate::errors::Error;
use sqlx::{Executor, MySqlPool};

pub trait Queryer<'c>: Executor<'c, Database = sqlx::MySql> + Copy {}

impl<'c> Queryer<'c> for &MySqlPool {}

pub async fn establish_connection() -> Result<sqlx::MySqlPool, Error> {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        log::warn!(target: "symfonia::db", "You did not specify `DATABASE_URL` environment variable, defaulting to in-memory SQLlite.");
        "sqlite::memory:".to_string()
    });

    let pool = sqlx::MySqlPool::connect(&db_url).await?;

    Ok(pool)
}
