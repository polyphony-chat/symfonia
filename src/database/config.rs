use crate::database::Queryer;
use polyphony_types::{
    config::{ConfigEntity, ConfigValue},
    errors::Error,
};
use sqlx::{AnyPool, Row};
use tokio::io::AsyncReadExt;

pub struct ConfigService;

impl ConfigService {
    pub async fn init<'c, C: Queryer<'c>>(db: C) -> Result<ConfigValue, Error> {
        let config = if let Ok(confg_path) = std::env::var("CONFIG_PATH") {
            if let Ok(mut f) = tokio::fs::File::open(&confg_path).await {
                let mut data = String::new();
                f.read_to_string(&mut data).await?;

                serde_json::from_str(&data)?
            } else {
                ConfigValue::default()
            }
        } else {
            let pairs = Self::collect_entities(db).await?;
            ConfigValue::from_pairs(pairs)
        };

        Ok(config)
    }

    pub async fn get_entity_by_key<'c, C: Queryer<'c>>(
        db: C,
        key: &str,
    ) -> Result<ConfigEntity, Error> {
        sqlx::query_as("SELECT * FROM config WHERE `key` = ?")
            .bind(key)
            .fetch_one(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn collect_entities<'c, C: Queryer<'c>>(db: C) -> Result<Vec<ConfigEntity>, Error> {
        sqlx::query_as("SELECT * FROM config")
            .fetch_all(db)
            .await
            .map_err(Error::SQLX)
    }
}
