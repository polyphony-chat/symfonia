use std::ops::{Deref, DerefMut};

use chorus::types::{PublicUser, Snowflake};
use serde::{Deserialize, Serialize};

use super::*;

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    #[sqlx(flatten)]
    #[serde(flatten)]
    inner: chorus::types::Relationship,
    pub from_id: Snowflake,
    pub nickname: Option<String>,
    pub user: PublicUser,
}

impl Deref for Relationship {
    type Target = chorus::types::Relationship;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Relationship {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Relationship {
    pub fn into_inner(self) -> chorus::types::Relationship {
        self.inner
    }

    pub fn as_inner(&self) -> &chorus::types::Relationship {
        &self.inner
    }
}
