use crate::{database::Queryer, errors::Error};
use chorus::types::Snowflake;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
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

impl Role {
    pub fn to_inner(self) -> chorus::types::RoleObject {
        self.inner
    }

    pub async fn create(
        db: &MySqlPool,
        id: Option<Snowflake>,
        guild_id: Snowflake,
        name: &str,
        color: i32,
        hoist: bool,
        managed: bool,
        mentionable: bool,
        permissions: &str,
        position: i32,
        icon: Option<String>,
        unicode_emoji: Option<String>,
    ) -> Result<Self, Error> {
        let role = Self {
            inner: chorus::types::RoleObject {
                id: if let Some(sf) = id {
                    sf
                } else {
                    Snowflake::default()
                },
                color,
                hoist,
                managed,
                mentionable,
                name: name.to_string(),
                permissions: permissions.to_string(),
                position,
                icon: icon.to_owned(),
                unicode_emoji: unicode_emoji.to_owned(),
                ..Default::default()
            },
            guild_id: guild_id.to_owned(),
        };

        sqlx::query("INSERT INTO roles (id, guild_id, name, color, hoist, managed, mentionable, permissions, position, icon, unicode_emoji) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(role.id)
            .bind(role.guild_id)
            .bind(&role.name)
            .bind(role.color)
            .bind(role.hoist)
            .bind(role.managed)
            .bind(role.mentionable)
            .bind(&role.permissions)
            .bind(role.position)
            .bind(&role.icon)
            .bind(&role.unicode_emoji)
            .execute(db)
            .await?;

        Ok(role)
    }

    pub async fn get_by_guild_id(db: &MySqlPool, guild_id: Snowflake) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * FROM roles WHERE guild_id = ?")
            .bind(guild_id)
            .fetch_all(db)
            .await
            .map_err(Error::SQLX)
    }
}
