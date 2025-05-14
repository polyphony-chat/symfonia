use crate::errors::Error;
use chorus::types::{Activity, ClientInfo, Snowflake, UserStatus};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::{PgPool, Row};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    #[sqlx(flatten)]
    inner: chorus::types::Session,
    pub index: Snowflake,
    pub user_id: Snowflake,
}

impl Session {
    pub async fn create(
        db: &PgPool,
        user: Snowflake,
        status: UserStatus,
        client_info: ClientInfo,
        activities: Vec<Activity>,
    ) -> Result<Self, Error> {
        // TODO: https://docs.discord.food/resources/presence#session-object ambiguity

        let session_id = Snowflake::generate();
        let res = sqlx::query("INSERT INTO sessions (user_id, session_id, activities, client_info, status) VALUES (?,?,?,?,?)")
            .bind(user)
            .bind(session_id)
            .bind(Json(&activities))
            .bind(Json(&client_info))
            .bind(status)
            .fetch_one(db)
            .await?;
        let index = res.get::<Snowflake, _>(0);

        Ok(Self {
            inner: chorus::types::Session {
                activities,
                client_info,
                session_id: session_id.to_string(),
                status,
            },
            index,
            user_id: user,
        })
    }

    pub async fn get_by_index(db: &PgPool, index: Snowflake) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM sessions WHERE index = ?")
            .bind(index)
            .fetch_optional(db)
            .await
            .map_err(Error::Sqlx)
    }

    pub async fn get_by_user(db: &PgPool, user: Snowflake) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * FROM sessions WHERE user_id = ?")
            .bind(user)
            .fetch_all(db)
            .await
            .map_err(Error::Sqlx)
    }

    pub async fn get_by_id(db: &PgPool, session_id: Snowflake) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM sessions WHERE session_id = ?")
            .bind(session_id)
            .fetch_optional(db)
            .await
            .map_err(Error::Sqlx)
    }

    pub fn into_inner(self) -> chorus::types::Session {
        self.inner
    }
}

impl Deref for Session {
    type Target = chorus::types::Session;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Session {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
