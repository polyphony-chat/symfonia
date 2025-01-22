use crate::{database::entities::guild_scheduled_event::GuildScheduledEvent, errors::Error};
use chorus::types::{Snowflake, StageInstancePrivacyLevel};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct StageInstance {
    #[serde(flatten)]
    #[sqlx(flatten)]
    inner: chorus::types::StageInstance,
}

impl StageInstance {
    pub async fn create(
        db: &PgPool,
        guild_id: Snowflake,
        channel_id: Snowflake,
        topic: &str,
        privacy_level: StageInstancePrivacyLevel,
        invite_code: &str,
        scheduled_event_id: Option<Snowflake>,
    ) -> Result<Self, Error> {
        let id = Snowflake::generate();
        let res = sqlx::query("INSERT INTO stage_instances (id, guild_id, channel_id, topic, privacy_level, invite_code, guild_scheduled_event_id) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(id)
            .bind(guild_id)
            .bind(channel_id)
            .bind(topic)
            .bind(privacy_level)
            .bind(invite_code)
            .bind(scheduled_event_id)
            .fetch_one(db)
            .await?;

        Ok(Self {
            inner: chorus::types::StageInstance {
                id,
                guild_id,
                channel_id,
                topic: topic.to_string(),
                privacy_level,
                discoverable_disabled: None,
                guild_scheduled_event_id: scheduled_event_id,
            },
        })
    }

    pub async fn get_by_id(db: &PgPool, id: Snowflake) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM stage_instances WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(Error::from)
    }

    pub async fn get_by_guild_id(db: &PgPool, guild_id: Snowflake) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * FROM stage_instances WHERE guild_id = ?")
            .bind(guild_id)
            .fetch_all(db)
            .await
            .map_err(Error::from)
    }

    pub async fn get_by_guild_and_channel_id(
        db: &PgPool,
        guild_id: Snowflake,
        channel_id: Snowflake,
    ) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * FROM stage_instances WHERE guild_id = ? AND channel_id = ?")
            .bind(guild_id)
            .bind(channel_id)
            .fetch_all(db)
            .await
            .map_err(Error::from)
    }

    pub async fn get_by_guild_scheduled_event_id(
        db: &PgPool,
        guild_scheduled_event_id: Snowflake,
    ) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM stage_instances WHERE guild_scheduled_event_id = ?")
            .bind(guild_scheduled_event_id)
            .fetch_optional(db)
            .await
            .map_err(Error::from)
    }

    pub async fn delete(&self, db: &PgPool) -> Result<(), Error> {
        sqlx::query("DELETE FROM stage_instances WHERE id = ?")
            .bind(self.id)
            .execute(db)
            .await
            .map(|_| ())
            .map_err(Error::from)
    }

    pub async fn get_scheduled_event(
        &self,
        db: &PgPool,
    ) -> Result<Option<GuildScheduledEvent>, Error> {
        if let Some(id) = self.guild_scheduled_event_id {
            GuildScheduledEvent::get_by_id(db, id).await
        } else {
            Ok(None)
        }
    }

    pub fn into_inner(self) -> chorus::types::StageInstance {
        self.inner
    }

    pub fn to_inner(&self) -> chorus::types::StageInstance {
        self.inner.clone()
    }
}

impl Deref for StageInstance {
    type Target = chorus::types::StageInstance;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for StageInstance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
