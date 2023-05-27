use chorus::types::Snowflake;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Sticker {
    inner: chorus::types::Sticker,
    pub user_id: Option<Snowflake>,
}

impl Deref for Sticker {
    type Target = chorus::types::Sticker;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Sticker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
