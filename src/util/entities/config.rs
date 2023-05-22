use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

use crate::errors::Error;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ConfigEntity {
    pub key: String,
    pub value: Value,
}

impl ConfigEntity {
    pub fn as_string(&self) -> String {
        self.value
            .as_str()
            .expect("value is not a string")
            .to_string()
    }

    pub fn as_bool(&self) -> bool {
        self.value.as_bool().expect("value is not a boolean")
    }

    pub fn as_int(&self) -> i64 {
        self.value.as_i64().expect("value is not a number")
    }

    pub async fn get_by_key(conn: &mut sqlx::MySqlConnection, key: &str) -> Result<Self, Error> {
        sqlx::query_as("SELECT * FROM config WHERE `key` = ?")
            .bind(key)
            .fetch_one(conn)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn collect(conn: &mut sqlx::MySqlConnection) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * FROM config")
            .fetch_all(conn)
            .await
            .map_err(Error::SQLX)
    }
}
