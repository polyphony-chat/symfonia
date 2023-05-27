use chorus::types::Snowflake;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    #[sqlx(flatten)]
    inner: chorus::types::Message,
    pub author_id: Snowflake,
}

impl Deref for Message {
    type Target = chorus::types::Message;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Message {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
