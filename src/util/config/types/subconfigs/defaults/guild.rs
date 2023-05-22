use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuildDefaults {
    pub max_presences: u64,
    pub max_video_channel_users: u16,
    pub afk_timeout: u16,
    pub default_message_notification: u8,
    pub explicit_content_filter: u8,
}

impl Default for GuildDefaults {
    fn default() -> Self {
        Self {
            max_presences: 250_000,
            max_video_channel_users: 200,
            afk_timeout: 300,
            default_message_notification: 1,
            explicit_content_filter: 0,
        }
    }
}
