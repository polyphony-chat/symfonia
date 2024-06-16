use std::ops::{Deref, DerefMut};

use chorus::types::{
    ChannelMessagesAnchor, ChannelModifySchema, ChannelType, CreateChannelInviteSchema, InviteType,
    MessageSendSchema, Snowflake,
};
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, types::Json};

use crate::{
    database::entities::{GuildMember, invite::Invite, message::Message, read_state::ReadState},
    errors::{ChannelError, Error, GuildError},
};

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

    pub async fn get_invites(&self, db: &MySqlPool) -> Result<Vec<Invite>, Error> {
        Invite::get_by_channel(db, self.id).await
    }

    pub async fn create_message(
        &mut self,
        db: &MySqlPool,
        payload: MessageSendSchema,
        author_id: Snowflake,
    ) -> Result<Message, Error> {
        let mut message = Message::create(db, payload, self.guild_id, self.id, author_id).await?;

        self.last_message_id = Some(message.id);
        self.save(db).await?;

        // TODO: emit events
        // TODO: Get partial GuildMember?
        if let Some(mut read_state) =
            ReadState::get_by_user_and_channel(db, self.id, author_id).await?
        {
            read_state.last_message_id = Some(message.id);
            read_state.save(db).await?;
        } else {
            ReadState::create(db, self.id, author_id, Some(message.id)).await?;
        }

        if let Some(guild_id) = self.guild_id {
            let mut member = GuildMember::get_by_id(db, author_id, guild_id)
                .await?
                .ok_or(Error::Guild(GuildError::MemberNotFound))?;

            // TODO: Update guild member last_message_id
        }

        message.populate_relations(db).await?;

        Ok(message)
    }

    pub async fn get_messages(
        &self,
        db: &MySqlPool,
        anchor: Option<ChannelMessagesAnchor>,
        limit: i32,
    ) -> Result<Vec<Message>, Error> {
        let anchor = anchor.unwrap_or(ChannelMessagesAnchor::Before(
            self.last_message_id
                .ok_or(Error::Channel(ChannelError::InvalidMessage))?,
        )); // TODO: Make this better
        let mut messages = Message::get_by_channel_id(db, self.id, anchor, limit).await?;
        if let Some(latest_message) =
            Message::get_by_id(db, self.id, self.last_message_id.unwrap()).await?
        {
            messages.push(latest_message);
        }
        Ok(messages)
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
        sqlx::query("UPDATE channels SET name = ?, topic = ?, nsfw = ?, position = ?, permission_overwrites = ?, rate_limit_per_user = ?, parent_id = ?, bitrate = ?, icon = ?, user_limit = ?, rtc_region = ?, default_auto_archive_duration = ?, default_reaction_emoji = ?, flags = ?, default_thread_rate_limit_per_user = ?, video_quality_mode = ?, channel_type = ?, last_message_id = ? WHERE id = ?")
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
            .bind(&self.last_message_id)
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

    pub fn is_text(&self) -> bool {
        self.channel_type == ChannelType::GuildText
            || self.channel_type == ChannelType::Dm
            || self.channel_type == ChannelType::GroupDm
    }

    pub fn is_writeable(&self) -> bool {
        !(self.channel_type == ChannelType::GuildCategory
            || self.channel_type == ChannelType::GuildStageVoice
            || self.channel_type == ChannelType::VoicelessWhiteboard)
    }
}
