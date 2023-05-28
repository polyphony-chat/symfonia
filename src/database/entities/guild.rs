use crate::{
    database::{
        entities::{Channel, Config, Role},
        Queryer,
    },
    errors::Error,
};
use chorus::types::{ChannelType, Snowflake, WelcomeScreenObject};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct Guild {
    #[sqlx(flatten)]
    inner: chorus::types::Guild,
    pub member_count: Option<i32>,
    pub presence_count: Option<i32>,
    pub unavailable: bool,
    pub parent: Option<String>,
    pub template_id: Option<Snowflake>,
    pub nsfw: bool,
}

impl Deref for Guild {
    type Target = chorus::types::Guild;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Guild {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Guild {
    pub async fn create<'c, C: Queryer<'c>>(
        db: C,
        cfg: &Config,
        name: &str,
        icon: Option<String>,
        owner_id: &Snowflake,
        channels: Vec<Channel>,
    ) -> Result<Self, Error> {
        let guild = Self {
            inner: chorus::types::Guild {
                name: name.to_string(),
                icon: Default::default(), // TODO: Handle guild Icon
                owner_id: Some(owner_id.to_owned()),
                preferred_locale: Some("en-US".to_string()),
                system_channel_flags: Some(4),
                welcome_screen: Some(sqlx::types::Json(WelcomeScreenObject {
                    enabled: false,
                    description: Some("Fill in your description".to_string()),
                    welcome_channels: Vec::default(),
                })),
                afk_timeout: Some(cfg.defaults.guild.afk_timeout as u8),
                default_message_notifications: Some(
                    cfg.defaults.guild.default_message_notifications,
                ),
                explicit_content_filter: Some(cfg.defaults.guild.explicit_content_filter),
                features: Default::default(), // TODO: cfg.guild.default_features
                max_members: Some(cfg.limits.guild.max_members),
                max_presences: Some(cfg.defaults.guild.max_presences),
                max_video_channel_users: Some(cfg.defaults.guild.max_video_channel_users as u8),
                region: Some(cfg.regions.default.clone()),
                ..Default::default()
            },
            ..Default::default()
        };

        let everyone = Role::create(
            db,
            Some(guild.id.clone()),
            &guild.id,
            "@everyone",
            0.,
            false,
            true,
            false,
            "2251804225",
            0,
            None,
            None,
        )
        .await?;

        let channels = if channels.is_empty() {
            vec![
                Channel::create(
                    db,
                    ChannelType::GuildText,
                    Some("general".to_string()),
                    false,
                    Some(guild.id.to_owned()),
                    None,
                    false,
                    false,
                    false,
                    false,
                )
                .await?,
            ]
        } else {
            channels
        };

        Ok(guild)
    }

    pub async fn get_by_id<'c, C: Queryer<'c>>(
        db: C,
        id: &Snowflake,
    ) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM guilds WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GuildBan {
    inner: chorus::types::GuildBan,
    pub id: Snowflake,
    pub executor_id: Snowflake,
    pub ip: String,
}

impl Deref for GuildBan {
    type Target = chorus::types::GuildBan;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for GuildBan {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
