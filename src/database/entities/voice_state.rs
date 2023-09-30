use chorus::types::Snowflake;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct VoiceState {
    #[sqlx(flatten)]
    inner: chorus::types::VoiceState,
    pub id: Snowflake,
}

impl Deref for VoiceState {
    type Target = chorus::types::VoiceState;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for VoiceState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
