use std::ops::{Deref, DerefMut};

use chorus::types::{
    ChannelModifySchema, ChannelType, CreateChannelInviteSchema, InviteType, Snowflake,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use sqlx::types::Json;

use crate::database::entities::invite::Invite;
use crate::errors::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Channel {
    #[sqlx(flatten)]
    pub(crate) inner: chorus::types::Channel,
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
    pub fn to_inner(self) -> chorus::types::Channel {
        self.inner
    }

    pub async fn create(
        db: &MySqlPool,
        channel_type: ChannelType,
        name: Option<String>,
        nsfw: bool,
        guild_id: Option<Snowflake>,
        parent_id: Option<Snowflake>,
        exists_check: bool,
        permission_check: bool,
        event_emit: bool,
        name_checks: bool,
    ) -> Result<Self, Error> {
        if permission_check {
            todo!()
        }

        if name_checks {
            todo!()
        }

        match channel_type {
            ChannelType::GuildText | ChannelType::GuildNews | ChannelType::GuildVoice => {
                if parent_id.is_some() && exists_check {
                    todo!()
                }
            }
            ChannelType::Dm | ChannelType::GroupDm => {
                todo!() // TODO: No dms in a guild!
            }
            ChannelType::GuildCategory | ChannelType::Unhandled => {}
            ChannelType::GuildStore => {}
            _ => {}
        }

        // TODO: permission overwrites

        let channel = Self {
            inner: chorus::types::Channel {
                channel_type,
                name,
                nsfw: Some(nsfw),
                guild_id,
                ..Default::default()
            },
        };

        sqlx::query("INSERT INTO channels (id, type, name, nsfw, guild_id, parent_id, flags, default_thread_rate_limit_per_user, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, NOW())")
            .bind(channel.id)
            .bind(channel.channel_type)
            .bind(&channel.name)
            .bind(channel.nsfw)
            .bind(channel.guild_id)
            .bind(channel.parent_id)
            .bind(0)
            .bind(0)
            .execute(db)
            .await?;

        Ok(channel)
    }

    pub async fn get_by_id(db: &MySqlPool, id: Snowflake) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM channels WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn get_by_guild_id(db: &MySqlPool, guild_id: Snowflake) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * FROM channels WHERE guild_id = ?")
            .bind(guild_id)
            .fetch_all(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn delete(&self, db: &MySqlPool) -> Result<(), Error> {
        sqlx::query("DELETE FROM channels WHERE id = ?")
            .bind(self.id)
            .execute(db)
            .await
            .map(|_| ())
            .map_err(Error::from)
    }

    pub fn modify(&mut self, data: ChannelModifySchema) {
        self.name = data.name;
        self.topic = data.topic;
        self.nsfw = data.nsfw;
        self.position = data.position;
        self.permission_overwrites = data.permission_overwrites.map(Json);
        self.rate_limit_per_user = data.rate_limit_per_user;
        self.parent_id = data.parent_id;
        self.bitrate = data.bitrate;
        self.icon = data.icon;
        self.user_limit = data.user_limit;
        self.rtc_region = data.rtc_region;
        self.default_auto_archive_duration = data.default_auto_archive_duration;
        self.default_reaction_emoji = data.default_reaction_emoji.map(Json);
        self.flags = data.flags;
        self.default_thread_rate_limit_per_user = data.default_thread_rate_limit_per_user;
        self.video_quality_mode = data.video_quality_mode;

        if let Some(channel_type) = data.channel_type {
            self.channel_type = channel_type;
        }
    }

    pub async fn save(&self, db: &MySqlPool) -> Result<(), Error> {
        sqlx::query("UPDATE channels SET name = ?, topic = ?, nsfw = ?, position = ?, permission_overwrites = ?, rate_limit_per_user = ?, parent_id = ?, bitrate = ?, icon = ?, user_limit = ?, rtc_region = ?, default_auto_archive_duration = ?, default_reaction_emoji = ?, flags = ?, default_thread_rate_limit_per_user = ?, video_quality_mode = ?, channel_type = ? WHERE id = ?")
            .bind(&self.name)
            .bind(&self.topic)
            .bind(&self.nsfw)
            .bind(&self.position)
            .bind(&self.permission_overwrites)
            .bind(&self.rate_limit_per_user)
            .bind(&self.parent_id)
            .bind(&self.bitrate)
            .bind(&self.icon)
            .bind(&self.user_limit)
            .bind(&self.rtc_region)
            .bind(&self.default_auto_archive_duration)
            .bind(&self.default_reaction_emoji)
            .bind(&self.flags)
            .bind(&self.default_thread_rate_limit_per_user)
            .bind(&self.video_quality_mode)
            .bind(&self.channel_type)
            .bind(&self.id)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn create_invite(
        &self,
        db: &MySqlPool,
        payload: CreateChannelInviteSchema,
        inviter_id: Option<Snowflake>,
    ) -> Result<Invite, Error> {
        Invite::create(db, payload, Some(self.id), inviter_id, InviteType::Guild).await
    }
}
