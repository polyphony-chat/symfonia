use crate::database::entities::{Guild, Role, User};
use crate::errors::{Error, GuildError, UserError};
use chorus::types::{PublicUser, Snowflake, UserGuildSettingsUpdate};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

#[derive(Debug, Default, Clone, Serialize, Deserialize, FromRow)]
pub struct GuildMember {
    #[serde(flatten)]
    #[sqlx(flatten)]
    inner: chorus::types::GuildMember,
    pub index: i32,
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub settings: sqlx::types::Json<UserGuildSettingsUpdate>,
    #[sqlx(skip)]
    pub user_data: User,
}

impl Deref for GuildMember {
    type Target = chorus::types::GuildMember;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for GuildMember {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl GuildMember {
    pub async fn create(db: &sqlx::MySqlPool, user: &User, guild: &Guild) -> Result<Self, Error> {
        let public = user.to_public_user();
        let mut member = Self {
            inner: chorus::types::GuildMember {
                user: Some(public),
                nick: None,
                avatar: None, // TODO
                roles: vec![guild.id],
                joined_at: chrono::Utc::now(),
                ..Default::default()
            },
            index: 0,
            id: user.id,
            guild_id: guild.id,
            settings: Default::default(),
            user_data: user.to_owned(),
        };

        let res = sqlx::query("INSERT INTO members (id, guild_id, joined_at, deaf, mute, pending, settings, bio) VALUES (?, ?, NOW(), 0, 0, 0, ?, ?)")
            .bind(user.id)
            .bind(guild.id)
            .bind(sqlx::types::Json(UserGuildSettingsUpdate::default()))
            .bind(user.bio.clone().unwrap_or_default())
            .execute(db)
            .await
            .map_err(Error::from)?;

        let index = res.last_insert_id();
        member.index = index as i32;

        sqlx::query("INSERT INTO member_roles (`index`, role_id) VALUES (?,?)")
            .bind(index)
            .bind(guild.id)
            .execute(db)
            .await?;

        Ok(member)
    }

    pub async fn get_by_id(
        db: &sqlx::MySqlPool,
        id: Snowflake,
        guild_id: Snowflake,
    ) -> Result<Option<Self>, Error> {
        let mut member: Self =
            sqlx::query_as("SELECT * FROM members WHERE id = ? AND guild_id = ?")
                .bind(id)
                .bind(guild_id)
                .fetch_optional(db)
                .await
                .map_err(Error::from)?
                .ok_or(Error::Guild(GuildError::MemberNotFound))?;

        // TODO: combine these queries with a JOIN

        let user = User::get_by_id(db, id)
            .await?
            .ok_or(Error::User(UserError::InvalidUser))?;

        member.user_data = user;

        Ok(Some(member))
    }
}
