use crate::{database::Queryer, errors::Error};
use chorus::types::Snowflake;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Guild {
    #[sqlx(flatten)]
    inner: chorus::types::Guild,
    #[sqlx(rename = "features")]
    pub features_array: String, // This is actually a 'simple array', delimited by commas
    pub member_count: Option<u64>,
    pub presence_count: Option<u64>,
    pub unavailable: bool,
    pub parent: Option<String>,
    pub template_id: Option<Snowflake>,
    pub nsfw: bool,
}

impl Deref for Guild {
    type Target = chorus::types::Guild;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Guild {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Guild {
    pub async fn get_by_id<'c, C: Queryer<'c>>(
        db: C,
        id: &Snowflake,
    ) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM guilds WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GuildBan {
    inner: chorus::types::GuildBan,
    pub id: Snowflake,
    pub executor_id: Snowflake,
}

impl Deref for GuildBan {
    type Target = chorus::types::GuildBan;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for GuildBan {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
