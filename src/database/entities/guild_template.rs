use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::errors::Error;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GuildTemplate {
    #[serde(flatten)]
    #[sqlx(flatten)]
    inner: chorus::types::GuildTemplate,
}

impl Deref for GuildTemplate {
    type Target = chorus::types::GuildTemplate;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for GuildTemplate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl GuildTemplate {
    pub async fn get_by_code(db: &MySqlPool, code: &str) -> Result<Option<GuildTemplate>, Error> {
        sqlx::query_as("SELECT * FROM guild_templates WHERE code = ?")
            .bind(code)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }

    pub fn into_inner(self) -> chorus::types::GuildTemplate {
        self.inner
    }
}
