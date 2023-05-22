use crate::errors::Error;

pub async fn establish_connection() -> Result<sqlx::AnyPool, Error> {
    let db_url = std::env::var("DATABASE_URL").unwrap_or({
        log::warn!(target: "spacebar::db", "You did not specify `DATABASE_URL` environment variable, defaulting to in-memory SQLlite.");
        "sqlite::memory:".to_string()
    });

    let pool = sqlx::AnyPool::connect(&db_url).await?;

    Ok(pool)
}
