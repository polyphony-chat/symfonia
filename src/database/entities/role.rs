use crate::{database::Queryer, errors::Error};
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

impl Role {
    pub async fn create<'c, C: Queryer<'c>>(
        db: C,
        id: Option<Snowflake>,
        guild_id: &Snowflake,
        name: &str,
        color: f64,
        hoist: bool,
        managed: bool,
        mentionable: bool,
        permissions: &str,
        position: u16,
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

        sqlx::query("INSERT INTO roles (id, guild_id, name, color, hoist, managed, mentionable, permissions, position, icon, unicode_emoji) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(&role.id)
            .bind(&role.guild_id)
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
}
