use chrono::{serde::ts_milliseconds_option, Utc};
use serde::{Deserialize, Serialize};

use crate::util::Snowflake;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    #[default]
    Online,
    Offline,
    Dnd,
    Idle,
    Invisible,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum UserTheme {
    #[default]
    Dark,
    Light,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserSettings {
    pub id: String,
    pub afk_timeout: u16,
    pub allow_accessibility_detection: bool,
    pub animate_emoji: bool,
    pub animate_stickers: u8,
    pub contact_sync_enabled: bool,
    pub convert_emoticons: bool,
    pub custom_status: Option<CustomStatus>,
    pub default_guilds_restricted: bool,
    pub detect_platform_accounts: bool,
    pub developer_mode: bool,
    pub disable_games_tab: bool,
    pub enable_tts_command: bool,
    pub explicit_content_filter: u8,
    pub friend_source_flags: FriendSourceFlags,
    pub gateway_connected: bool,
    pub gif_auto_play: bool,
    pub guild_folders: Vec<GuildFolder>,
    pub guild_positions: Vec<String>,
    pub inline_attachment_media: bool,
    pub inline_embed_media: bool,
    pub locale: String,
    pub message_display_compact: bool,
    pub native_phone_integration_enabled: bool,
    pub render_embeds: bool,
    pub render_reactions: bool,
    pub restricted_guilds: Vec<String>,
    pub show_current_game: bool,
    pub status: UserStatus,
    pub stream_notifications_enabled: bool,
    pub theme: UserTheme,
    pub timezone_offset: i16,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            id: Snowflake::generate().to_string(),
            afk_timeout: 3600,
            allow_accessibility_detection: true,
            animate_emoji: true,
            animate_stickers: 0,
            contact_sync_enabled: false,
            convert_emoticons: false,
            custom_status: None,
            default_guilds_restricted: false,
            detect_platform_accounts: false,
            developer_mode: true,
            disable_games_tab: true,
            enable_tts_command: false,
            explicit_content_filter: 0,
            friend_source_flags: FriendSourceFlags::default(),
            gateway_connected: false,
            gif_auto_play: false,
            guild_folders: Vec::new(),
            guild_positions: Vec::new(),
            inline_attachment_media: true,
            inline_embed_media: true,
            locale: "en-US".to_string(),
            message_display_compact: false,
            native_phone_integration_enabled: true,
            render_embeds: true,
            render_reactions: true,
            restricted_guilds: Vec::new(),
            show_current_game: true,
            status: UserStatus::Online,
            stream_notifications_enabled: false,
            theme: UserTheme::Dark,
            timezone_offset: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomStatus {
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>,
    #[serde(with = "ts_milliseconds_option")]
    pub expires_at: Option<chrono::DateTime<Utc>>,
    pub text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FriendSourceFlags {
    pub all: bool,
}

impl Default for FriendSourceFlags {
    fn default() -> Self {
        Self { all: true }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuildFolder {
    pub color: u32,
    pub guild_ids: Vec<String>,
    pub id: u16,
    pub name: String,
}
