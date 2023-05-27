use crate::{database::Queryer, errors::Error};
use chorus::types::Snowflake;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Channel {
    #[sqlx(flatten)]
    inner: chorus::types::Channel,
}

impl Deref for Channel {
    type Target = chorus::types::Channel;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Channel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Channel {
    pub async fn get_by_id<'c, C: Queryer<'c>>(
        db: C,
        id: &Snowflake,
    ) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM channels WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }
}
