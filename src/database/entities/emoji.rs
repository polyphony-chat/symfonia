use std::ops::{Deref, DerefMut};

use chorus::types::Snowflake;
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, Row};

use crate::{database::entities::User, errors::Error};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Emoji {
    #[sqlx(flatten)]
    inner: chorus::types::Emoji,
    pub guild_id: Snowflake,
    pub user_id: Option<Snowflake>,
}

impl Deref for Emoji {
    type Target = chorus::types::Emoji;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Emoji {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Emoji {
    pub async fn create(
        db: &MySqlPool,
        guild_id: Snowflake,
        user_id: Option<Snowflake>,
        name: &str,
        animated: bool,
        managed: bool,
        require_colons: bool,
        role_ids: Vec<Snowflake>,
    ) -> Result<Self, Error> {
        let query = sqlx::query(
            "INSERT INTO emojis (id, guild_id, user_id, name, animated, managed, require_colons, available) VALUES (?,?,?,?,?,?,1)",
        );

        let id = Snowflake::generate();

        let mut user = None;
        if let Some(user_id) = user_id {
            user = User::get_by_id(db, user_id).await?.map(|u| u.to_inner());
        }

        query
            .bind(id)
            .bind(guild_id)
            .bind(user_id)
            .bind(name)
            .bind(animated)
            .bind(managed)
            .bind(require_colons)
            .execute(db)
            .await
            .map_err(Error::SQLX)?;

        Ok(Self {
            inner: chorus::types::Emoji {
                id,
                name: Some(name.to_string()),
                animated: Some(animated),
                managed: Some(managed),
                require_colons: Some(require_colons),
                roles: Some(sqlx::types::Json(role_ids)),
                user,
                available: Some(true),
            },
            guild_id,
            user_id,
        })
    }

    pub async fn get_by_id(db: &MySqlPool, id: Snowflake) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM emojis WHERE id = ? AND guild_id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn get_by_guild(db: &MySqlPool, guild_id: Snowflake) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * FROM emojis WHERE guild_id = ?")
            .bind(guild_id)
            .fetch_all(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn count(db: &MySqlPool, guild_id: Snowflake) -> Result<i32, Error> {
        sqlx::query("SELECT COUNT(*) FROM emojis WHERE guild_id =?")
            .bind(guild_id)
            .fetch_one(db)
            .await
            .map_err(Error::SQLX)
            .map(|r| r.get::<i32, _>(0))
    }

    pub async fn save(&mut self, db: &MySqlPool) -> Result<(), Error> {
        sqlx::query(
            "UPDATE emojis SET name = ?, require_colons = ?, roles =? WHERE id = ? AND guild_id = ?",
        )
           .bind(&self.name)
           .bind(self.require_colons)
           .bind(&self.roles)
           .bind(self.id)
           .bind(self.guild_id)
           .execute(db)
           .await
           .map_err(Error::SQLX)?;

        Ok(())
    }

    pub async fn delete(self, db: &MySqlPool) -> Result<(), Error> {
        sqlx::query("DELETE FROM emojis WHERE id = ?")
            .bind(self.id)
            .execute(db)
            .await
            .map_err(Error::SQLX)
            .map(|_| ())
    }
}
