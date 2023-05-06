use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{database::entities::user_setting::UserSettings, util::Snowflake};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub accent_color: Option<u16>,
    pub banner: Option<String>,
    pub theme_colors: Option<Vec<u16>>,
    pub pronouns: Option<String>,
    pub phone: Option<String>,
    pub desktop: bool,
    pub mobile: bool,
    pub premium: bool,
    pub premium_type: usize,
    pub bot: bool,
    pub bio: String,
    pub system: bool,
    pub nsfw_allowed: bool,
    pub mfa_enabled: bool,
    pub webauthn_enabled: bool,
    #[serde(skip)]
    pub totp_secret: Option<String>,
    #[serde(skip)]
    pub totp_last_ticket: Option<String>,
    pub created_at: NaiveDateTime,
    pub premium_since: Option<NaiveDateTime>,
    pub verified: bool,
    pub disabled: bool,
    pub deleted: bool,
    pub email: Option<String>,
    pub flags: String,
    pub public_flags: usize,
    pub purchased_flags: usize,
    pub premium_usage_flags: usize,
    pub rights: String,
    pub relationship_ids: Vec<String>,
    pub connected_account_ids: Vec<String>,
    pub data: UserData,
    pub fingerprints: Vec<String>,
    pub settings: UserSettings,
    pub extended_settings: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserData {
    pub valid_tokens_since: NaiveDateTime,
    pub hash: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Snowflake::generate().to_string(),
            username: String::new(),
            discriminator: String::new(),
            avatar: None,
            accent_color: None,
            banner: None,
            theme_colors: None,
            pronouns: None,
            phone: None,
            desktop: false,
            mobile: false,
            premium: false,
            premium_type: 0,
            bot: false,
            bio: String::new(),
            system: false,
            nsfw_allowed: false,
            mfa_enabled: false,
            webauthn_enabled: false,
            totp_secret: None,
            totp_last_ticket: None,
            created_at: Utc::now().naive_utc(),
            premium_since: None,
            verified: false,
            disabled: false,
            deleted: false,
            email: None,
            flags: String::from("0"),
            public_flags: 0,
            purchased_flags: 0,
            premium_usage_flags: 0,
            rights: String::new(),
            relationship_ids: Vec::new(),
            connected_account_ids: Vec::new(),
            data: todo!(),
            fingerprints: Vec::new(),
            settings: UserSettings::default(),
            extended_settings: String::from("{}"),
        }
    }
}

pub struct PublicUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub accent_color: Option<u16>,
    pub banner: Option<String>,
    pub theme_colors: Option<Vec<u16>>,
    pub pronouns: Option<String>,
    pub bot: bool,
    pub bio: String,
    pub premium_type: usize,
    pub premium_since: Option<NaiveDateTime>,
    pub public_flags: usize,
}

impl From<User> for PublicUser {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            discriminator: value.discriminator,
            avatar: value.avatar,
            accent_color: value.accent_color,
            banner: value.banner,
            theme_colors: value.theme_colors,
            pronouns: value.pronouns,
            bot: value.bot,
            bio: value.bio,
            premium_type: value.premium_type,
            premium_since: value.premium_since,
            public_flags: value.public_flags,
        }
    }
}
