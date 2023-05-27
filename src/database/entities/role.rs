use chorus::types::Snowflake;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Role {
    #[sqlx(flatten)]
    inner: chorus::types::RoleObject,
    pub guild_id: Snowflake,
}

impl Deref for Role {
    type Target = chorus::types::RoleObject;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Role {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
