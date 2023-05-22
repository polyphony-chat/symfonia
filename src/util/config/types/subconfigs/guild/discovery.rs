use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoverConfiguration {
    pub show_all_guilds: bool,
    pub use_recommendation: bool,
    pub offset: u16,
    pub limit: u16,
}

impl Default for DiscoverConfiguration {
    fn default() -> Self {
        Self {
            show_all_guilds: false,
            use_recommendation: false,
            offset: 0,
            limit: 24,
        }
    }
}
