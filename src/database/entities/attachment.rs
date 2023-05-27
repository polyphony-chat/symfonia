use chorus::types::Snowflake;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Attachment {
    #[sqlx(flatten)]
    inner: chorus::types::Attachment,
    pub message_id: Option<Snowflake>,
}

impl Deref for Attachment {
    type Target = chorus::types::Attachment;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Attachment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
