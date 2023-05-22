use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuildLimits {
    pub max_roles: u16,
    pub max_emojis: u16,
    pub max_members: u64,
    pub max_channels: u32,
    pub max_channels_in_category: u32,
}

impl Default for GuildLimits {
    fn default() -> Self {
        Self {
            max_roles: 1000,
            max_emojis: 20_000,
            max_members: 25_000_000,
            max_channels: 65_535,
            max_channels_in_category: 65_535,
        }
    }
}
