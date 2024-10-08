use std::ops::{Deref, DerefMut};

use bigdecimal::BigDecimal;
use chorus::types::{PublicUser, Snowflake};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::Error;
use crate::QUERY_UPPER_LIMIT;

use super::*;

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    #[sqlx(flatten)]
    pub(crate) inner: chorus::types::Relationship,
    pub from_id: Snowflake,
    #[sqlx(skip)]
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

    pub fn as_inner_mut(&mut self) -> &mut chorus::types::Relationship {
        &mut self.inner
    }

    // TODO: Write tests for this
    pub async fn get_by_from_id(from_id: Snowflake, db: &PgPool) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * from relationships WHERE from_id = $1 LIMIT $2")
            .bind(from_id)
            .bind(QUERY_UPPER_LIMIT)
            .fetch_all(db)
            .await
            .map_err(Error::from)
    }
}
