pub mod entities;

use crate::errors::Error;
use sqlx::{AnyPool, Database, Executor, MySqlPool, SqlitePool};

pub trait Queryer<'c, DB: Database>: Executor<'c, Database = DB> {}

impl<'c> Queryer<'c, sqlx::MySql> for &MySqlPool {}

impl<'c> Queryer<'c, sqlx::Sqlite> for &SqlitePool {}

impl<'c> Queryer<'c, sqlx::Any> for &AnyPool {}

pub async fn establish_connection() -> Result<sqlx::MySqlPool, Error> {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        log::warn!(target: "symfonia::db", "You did not specify `DATABASE_URL` environment variable, defaulting to in-memory SQLlite.");
        "sqlite::memory:".to_string()
    });

    let pool = MySqlPool::connect(&db_url).await?;
    //install_default_drivers();

    Ok(pool)
}
