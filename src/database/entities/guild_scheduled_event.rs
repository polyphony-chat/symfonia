use crate::errors::Error;
use chorus::types::Snowflake;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct GuildScheduledEvent {
    #[serde(flatten)]
    #[sqlx(flatten)]
    inner: chorus::types::GuildScheduledEvent,
}

impl GuildScheduledEvent {
    pub async fn create(
        db: &PgPool,
        guild_id: Snowflake,
        channel_id: impl Into<Option<Snowflake>> + Copy,
        creator_id: impl Into<Option<Snowflake>> + Copy,
        name: &str,
        description: &str,
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: impl Into<Option<chrono::DateTime<chrono::Utc>>> + Copy,
        privacy_level: chorus::types::GuildScheduledEventPrivacyLevel,
        entity_type: chorus::types::GuildScheduledEventEntityType,
        entity_id: impl Into<Option<Snowflake>> + Copy,
        status: chorus::types::GuildScheduledEventStatus,
        image: impl Into<Option<String>> + Clone,
        location: impl Into<Option<String>> + Clone,
    ) -> Result<Self, Error> {
        let id = Snowflake::generate();
        let res = sqlx::query("INSERT INTO guild_scheduled_events (id, guild_id, channel_id, creator_id, name, description, scheduled_start_time, scheduled_end_time, privacy_level, status, entity_type, entity_id, location, image) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(id)
            .bind(guild_id)
            .bind(channel_id.into())
            .bind(creator_id.into())
            .bind(name)
            .bind(description)
            .bind(start_time)
            .bind(end_time.into())
            .bind(privacy_level)
            .bind(entity_type)
            .bind(entity_id.into())
            .bind(status)
            .bind(location.into())
            .bind(image.clone().into())
            .execute(db)
            .await?;

        Ok(Self {
            inner: chorus::types::GuildScheduledEvent {
                id,
                guild_id,
                channel_id: channel_id.into(),
                creator_id: creator_id.into(),
                name: name.to_string(),
                description: description.to_string(),
                scheduled_start_time: start_time,
                scheduled_end_time: end_time.into(),
                privacy_level,
                status,
                entity_type,
                entity_id: entity_id.into(),
                entity_metadata: None,
                creator: None,
                user_count: None,
                image: image.into(),
            },
        })
    }

    pub async fn get_by_id(db: &PgPool, id: Snowflake) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM guild_scheduled_events WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(Error::from)
    }

    pub async fn get_by_guild_id(db: &PgPool, guild_id: Snowflake) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * FROM guild_scheduled_events WHERE guild_id = ?")
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
        sqlx::query_as("SELECT * FROM guild_scheduled_events WHERE guild_id = ? AND channel_id = ?")
            .bind(guild_id)
            .bind(channel_id)
            .fetch_all(db)
            .await
            .map_err(Error::from)
    }

    pub async fn get_by_creator_id(db: &PgPool, creator_id: Snowflake) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * FROM guild_scheduled_events WHERE creator_id = ?")
            .bind(creator_id)
            .fetch_all(db)
            .await
            .map_err(Error::from)
    }

    pub async fn delete(&self, db: &PgPool) -> Result<(), Error> {
        sqlx::query("DELETE FROM guild_scheduled_events WHERE id = ?")
            .bind(self.id)
            .execute(db)
            .await
            .map(|_| ())
            .map_err(Error::from)
    }

    pub fn into_inner(self) -> chorus::types::GuildScheduledEvent {
        self.inner
    }

    pub fn to_inner(&self) -> chorus::types::GuildScheduledEvent {
        self.inner.clone()
    }
}

impl Deref for GuildScheduledEvent {
    type Target = chorus::types::GuildScheduledEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for GuildScheduledEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
