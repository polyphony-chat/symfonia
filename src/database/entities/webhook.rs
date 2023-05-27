use chorus::types::Snowflake;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Webhook {
    #[sqlx(flatten)]
    inner: chorus::types::Webhook,
    pub source_guild_id: Snowflake,
    pub user_id: Snowflake,
}

impl Deref for Webhook {
    type Target = chorus::types::Webhook;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Webhook {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
