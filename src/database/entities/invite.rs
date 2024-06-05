use std::ops::{Deref, DerefMut};

use chorus::types::{CreateChannelInviteSchema, InviteType, Snowflake};
use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::database::entities::{Channel, Guild, User};
use crate::errors::{ChannelError, Error, GuildError};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Invite {
    #[serde(flatten)]
    #[sqlx(flatten)]
    inner: chorus::types::Invite,
    #[serde(skip)]
    pub channel_id: Option<Snowflake>,
    #[serde(skip)]
    pub inviter_id: Option<Snowflake>,
    #[serde(skip)]
    pub target_user_id: Option<Snowflake>,
    pub vanity_url: Option<bool>,
}

impl Deref for Invite {
    type Target = chorus::types::Invite;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Invite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Invite {
    pub async fn create(
        db: &MySqlPool,
        data: CreateChannelInviteSchema,
        channel_id: Option<Snowflake>,
        inviter_id: Option<Snowflake>,
        invite_type: InviteType,
    ) -> Result<Self, Error> {
        let random_code = String::from_utf8(
            rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(8) // TODO: Make this configurable?
                .collect::<Vec<u8>>(),
        )?;

        let mut guild_id = None;
        if let Some(channel_id) = channel_id {
            let channel = Channel::get_by_id(db, channel_id)
                .await?
                .ok_or(Error::Channel(ChannelError::InvalidChannel))?;

            guild_id = channel.guild_id;
        }

        let expires_at = data
            .max_age
            .map(|age| Utc::now() + chrono::Duration::seconds(age as i64));

        let invite = Self {
            inner: chorus::types::Invite {
                approximate_member_count: None,
                approximate_presence_count: None,
                channel: None,
                code: random_code.to_owned(),
                created_at: Some(Utc::now()),
                expires_at,
                flags: data.flags,
                guild: None,
                guild_id,
                guild_scheduled_event: None,
                invite_type: Some(invite_type),
                inviter: None,
                max_age: data.max_age,
                max_uses: data.max_uses,
                stage_instance: None,
                target_application: None,
                target_type: data.target_type,
                target_user: None,
                temporary: data.temporary,
                uses: None,
            },
            channel_id,
            inviter_id,
            vanity_url: None,
            target_user_id: data.target_user_id,
        };

        /*
        code, type, temporary, uses, max_uses, max_age, created_at, expires_at, guild_id, channel_id, inviter_id, target_user_id, target_user_type, vanity_url, flags
         */

        sqlx::query("code, type, temporary, uses, max_uses, max_age, created_at, expires_at, guild_id, channel_id, inviter_id, target_user_id, target_user_type, vanity_url, flags) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(random_code)
            .bind(invite.invite_type)
            .bind(invite.temporary)
            .bind(invite.uses)
            .bind(invite.max_uses)
            .bind(invite.max_age)
            .bind(invite.created_at)
            .bind(invite.expires_at)
            .bind(invite.guild_id)
            .bind(invite.channel_id)
            .bind(invite.inviter_id)
            .bind(invite.target_user_id)
            .bind(invite.target_type)
            .bind(invite.vanity_url)
            .bind(invite.flags)
            .execute(db)
            .await?;

        Ok(invite)
    }

    pub async fn get_by_code(db: &MySqlPool, code: &str) -> Result<Option<Self>, Error> {
        let invite: Option<Self> = sqlx::query_as("SELECT * FROM invites WHERE code = ?")
            .bind(code)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)?;

        Ok(invite)
    }

    pub async fn get_by_guild(db: &MySqlPool, guild_id: Snowflake) -> Result<Vec<Self>, Error> {
        let guild = Guild::get_by_id(db, guild_id)
            .await?
            .ok_or(Error::Guild(GuildError::InvalidGuild))?;

        let mut invites = sqlx::query_as("SELECT * FROM invites WHERE guild_id = ?")
            .bind(guild_id)
            .fetch_all(db)
            .await
            .map_err(Error::SQLX)?;

        invites
            .iter_mut()
            .for_each(|invite: &mut Invite| invite.guild = Some(guild.clone().into_inner().into()));

        Ok(invites)
    }

    pub async fn get_by_channel(db: &MySqlPool, channel_id: Snowflake) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * FROM invites WHERE channel_id = ?")
            .bind(channel_id)
            .fetch_all(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn delete(&self, db: &MySqlPool) -> Result<(), Error> {
        sqlx::query("DELETE FROM invites WHERE code = ?")
            .bind(&self.code)
            .execute(db)
            .await
            .map(|_| ())
            .map_err(Error::SQLX)
    }

    pub async fn join(&mut self, db: &MySqlPool, user: &User) -> Result<(), Error> {
        if let Some(invite_type) = self.invite_type {
            match invite_type {
                InviteType::Guild => {
                    // TODO: Track what invite code a user used?
                    user.add_to_guild(
                        db,
                        self.guild_id
                            .ok_or(Error::Guild(GuildError::InvalidGuild))?,
                    )
                    .await?;
                    self.increase_uses(db).await?;
                }
                InviteType::GroupDm => todo!(),
                InviteType::Friend => todo!(),
            }
        } else {
            // TODO: Some form of handling for invites without an invite type?  maybe just default to guild?
        }

        let max_uses = self.max_uses.unwrap_or(0) as i32;
        if self.uses.unwrap_or(0) > max_uses && max_uses > 0 {
            self.delete(db).await?;
        }

        Ok(())
    }

    pub async fn increase_uses(&mut self, db: &MySqlPool) -> Result<(), Error> {
        self.uses = self.uses.map(|uses| uses + 1);
        sqlx::query("UPDATE invites SET uses = ? WHERE code = ?")
            .bind(&self.uses)
            .bind(&self.code)
            .execute(db)
            .await
            .map(|_| ())
            .map_err(Error::SQLX)
    }

    pub fn into_inner(self) -> chorus::types::Invite {
        self.inner
    }
}
