use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};

use chorus::types::{
    ChannelType, NSFWLevel, PremiumTier, PublicUser, Snowflake,
    SystemChannelFlags, types::guild_configuration::GuildFeaturesList, WelcomeScreenObject,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool, QueryBuilder, Row};

use crate::{
    database::{
        entities::{Channel, Config, GuildMember, Invite, Role, User},
        Queryer,
    },
    errors::{Error, GuildError, UserError},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct Guild {
    #[sqlx(flatten)]
    #[serde(flatten)]
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
    pub async fn create(
        db: &MySqlPool,
        cfg: &Config,
        name: &str,
        icon: Option<String>,
        owner_id: Snowflake,
        channels: Vec<Channel>,
    ) -> Result<Self, Error> {
        let mut guild = Self {
            inner: chorus::types::Guild {
                name: Some(name.to_string()),
                icon: Default::default(), // TODO: Handle guild Icon
                owner_id: Some(owner_id.to_owned()),
                preferred_locale: Some("en-US".to_string()),
                system_channel_flags: Some(
                    SystemChannelFlags::SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATIONS,
                ),
                welcome_screen: sqlx::types::Json(Some(WelcomeScreenObject {
                    enabled: false,
                    description: Some("Fill in your description".to_string()),
                    welcome_channels: Vec::default(),
                })),
                afk_timeout: Some(cfg.defaults.guild.afk_timeout as i32),
                default_message_notifications: Some(
                    cfg.defaults.guild.default_message_notifications,
                ),
                explicit_content_filter: Some(cfg.defaults.guild.explicit_content_filter),
                features: cfg.guild.default_features.clone().into(),
                max_members: Some(cfg.limits.guild.max_members as i32),
                max_presences: Some(cfg.defaults.guild.max_presences as i32),
                max_video_channel_users: Some(cfg.defaults.guild.max_video_channel_users as i32),
                region: Some(cfg.regions.default.clone()),
                premium_tier: Some(PremiumTier::Tier3), // Maybe make this configurable?
                nsfw_level: Some(NSFWLevel::Default),
                ..Default::default()
            },
            ..Default::default()
        };

        let features = guild
            .features
            .iter()
            .map(|x| x.to_str())
            .collect::<Vec<_>>()
            .join(",");
        println!("{:?}", features);
        let res = sqlx::query("INSERT INTO guilds (id, afk_timeout, default_message_notifications, explicit_content_filter, features, icon, max_members, max_presences, max_video_channel_users, name, owner_id, region, system_channel_flags, preferred_locale, welcome_screen, large, premium_tier, unavailable, widget_enabled, nsfw) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,0,?,0,0,?)")
            .bind(guild.id)
            .bind(guild.afk_timeout)
            .bind(guild.default_message_notifications)
            .bind(guild.explicit_content_filter)
            .bind(""/*&guild.features*/)
            .bind(""/*&guild.icon*/)
            .bind(500/*guild.max_members*/)
            .bind(500/*guild.max_presences*/)
            .bind(0/*guild.max_video_channel_users*/)
            .bind(&guild.name)
            .bind(guild.owner_id)
            .bind(""/*&guild.region*/)
            .bind(guild.system_channel_flags)
            .bind("en-US"/*&guild.preferred_locale*/)
            .bind("null"/*&guild.welcome_screen*/)
            .bind(2/*guild.premium_tier*/)
            .bind(true) // TODO: Do this better guild.nsfw_level
            .execute(db)
            .await.unwrap();
        log::debug!(target: "symfonia::guilds", "Created guild with id {}", guild.id);

        let everyone = Role::create(
            db,
            Some(guild.id),
            guild.id,
            "@everyone",
            0.,
            false,
            true,
            false,
            "2251804225", // 559623605571137?
            0,
            None,
            None,
        )
        .await?;

        let user = User::get_by_id(db, owner_id).await?.unwrap();

        user.add_to_guild(db, guild.id).await?;
        guild.owner = Some(true);

        guild.roles = Some(vec![everyone.to_inner()]);

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

        guild.channels = Some(channels.into_iter().map(|c| c.to_inner()).collect());

        Ok(guild)
    }

    pub async fn get_by_id(db: &MySqlPool, id: Snowflake) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM guilds WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn has_member(&self, db: &MySqlPool, user_id: Snowflake) -> Result<bool, Error> {
        sqlx::query_as("SELECT * FROM guild_members WHERE guild_id = ? AND user_id =?")
            .bind(self.id)
            .bind(user_id)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
            .map(|r: Option<GuildMember>| r.is_some())
    }

    pub async fn get_invites(&self, db: &MySqlPool) -> Result<Vec<Invite>, Error> {
        Invite::get_by_guild(db, self.id).await
    }

    pub async fn count(db: &MySqlPool) -> Result<i32, Error> {
        sqlx::query("SELECT COUNT(*) FROM guilds")
            .fetch_one(db)
            .await
            .map_err(Error::SQLX)
            .map(|r| r.get::<i32, _>(0))
    }

    pub async fn delete(self, db: &MySqlPool) -> Result<(), Error> {
        sqlx::query("DELETE FROM guilds WHERE id =?")
            .bind(self.id)
            .execute(db)
            .await
            .map_err(Error::SQLX)
            .map(|_| ())
    }

    pub fn into_inner(self) -> chorus::types::Guild {
        self.inner
    }

    pub fn to_inner(&self) -> &chorus::types::Guild {
        &self.inner
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GuildBan {
    #[sqlx(flatten)]
    inner: chorus::types::GuildBan,
    pub id: Snowflake,
    pub executor_id: Snowflake,
    pub guild_id: Snowflake,
    pub user_id: Snowflake,
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

impl GuildBan {
    pub async fn create(
        db: &MySqlPool,
        guild_id: Snowflake,
        user_id: Snowflake,
        executing_user_id: Snowflake,
        reason: impl Into<Option<String>>,
    ) -> Result<Self, Error> {
        let ban_id = Snowflake::default();
        let reason = reason.into();
        let user = GuildMember::get_by_id(db, user_id, guild_id)
            .await?
            .ok_or(Error::Guild(GuildError::MemberNotFound))?;

        sqlx::query("INSERT INTO guild_bans (id, guild_id, user_id, executor_id, reason, ip) VALUES (?,?,?,?,?,'127.0.0.1')") // TODO: Do something to get the users IP
            .bind(ban_id)
            .bind(guild_id)
            .bind(user_id)
            .bind(executing_user_id)
            .bind(&reason)
            .execute(db)
            .await
            .map_err(Error::SQLX)?;

        Ok(Self {
            inner: chorus::types::GuildBan {
                reason,
                user: user.user_data.to_public_user(),
            },
            guild_id,
            user_id,
            executor_id: executing_user_id,
            id: ban_id,
            ip: "127.0.0.1".to_string(),
        })
    }

    pub async fn builk_create(
        db: &MySqlPool,
        guild_id: Snowflake,
        user_ids: Vec<Snowflake>,
        executing_user_id: Snowflake,
        reason: impl Into<Option<String>>,
    ) -> Result<Vec<GuildBan>, Error> {
        let mut query_builder = QueryBuilder::new(
            "INSERT INTO guild_bans (id, guild_id, user_id, executor_id, reason, ip) VALUES ",
        );
        let mut rows = user_ids
            .into_iter()
            .map(|user_id| GuildBan {
                inner: chorus::types::GuildBan {
                    user: Default::default(),
                    reason: None,
                },
                id: Snowflake::default(),
                executor_id: executing_user_id,
                guild_id,
                user_id,
                ip: "127.0.0.1".to_string(), // TODO: Somehow get the users IP
            })
            .collect::<Vec<_>>();

        let reason = reason.into();

        query_builder.push_values(rows.iter(), |mut b, user| {
            b.push_bind(user.id)
                .push_bind(user.guild_id)
                .push_bind(user.user_id)
                .push_bind(user.executor_id)
                .push_bind(&reason)
                .push_bind("127.0.0.1");
        });

        let query = query_builder.build();

        query.execute(db).await?;

        for row in rows.iter_mut() {
            row.user = User::get_by_id(db, row.user_id)
                .await?
                .ok_or(Error::User(UserError::InvalidUser))?
                .to_public_user();
        }

        Ok(rows)
    }

    pub async fn populate_relations(&mut self, db: &MySqlPool) -> Result<(), Error> {
        let user = User::get_by_id(db, self.user_id)
            .await?
            .ok_or(Error::User(UserError::InvalidUser))?;
        self.user = user.to_public_user();
        Ok(())
    }

    pub async fn get_by_id(db: &MySqlPool, id: Snowflake) -> Result<Option<GuildBan>, Error> {
        sqlx::query_as("SELECT * FROM guild_bans WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn get_by_guild(
        db: &MySqlPool,
        guild_id: Snowflake,
        before: Option<Snowflake>,
        after: Option<Snowflake>,
        limit: Option<u16>,
    ) -> Result<Vec<GuildBan>, Error> {
        sqlx::query_as("SELECT * FROM bans WHERE (user_id < ? OR ? IS NULL) AND (user_id > ? OR ? IS NULL) AND guild_id = ? LIMIT IFNULL(?, 1000)")
            .bind(before)
            .bind(before)
            .bind(after)
            .bind(after)
            .bind(guild_id)
            .bind(limit)
            .fetch_all(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn get_by_user(
        db: &MySqlPool,
        guild_id: Snowflake,
        user_id: Snowflake,
    ) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM guild_bans WHERE user_id = ? AND guild_id = ?")
            .bind(user_id)
            .bind(guild_id)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn find_by_username(
        db: &MySqlPool,
        guild_id: Snowflake,
        search_term: &str,
        limit: u16,
    ) -> Result<Vec<Self>, Error> {
        let mut query = QueryBuilder::new("SELECT b.* FROM bans b JOIN members m ON b.user_id = m.id AND b.guild_id = m.guild_id JOIN users u ON b.user_id = u.id WHERE u.username LIKE '%");
        query.push_bind(search_term);
        query.push("%' AND b.guild_id = ");
        query.push_bind(guild_id);
        query.push(" LIMIT ");
        query.push_bind(limit);
        let query = query.build();
        let res = query.fetch_all(db).await?;
        let results = res
            .into_iter()
            .map(|r| GuildBan::from_row(&r).unwrap())
            .collect::<Vec<_>>();
        Ok(results)
    }

    pub async fn delete(self, db: &MySqlPool) -> Result<(), Error> {
        sqlx::query("DELETE FROM guild_bans WHERE id = ?")
            .bind(self.id)
            .execute(db)
            .await
            .map_err(Error::SQLX)?;
        Ok(())
    }

    pub fn into_inner(self) -> chorus::types::GuildBan {
        self.inner
    }
}
