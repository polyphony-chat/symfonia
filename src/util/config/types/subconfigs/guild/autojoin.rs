use serde::{Deserialize, Serialize};

use crate::util::Snowflake;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoJoinConfiguration {
    pub enabled: bool,
    pub guilds: Vec<Snowflake>,
    pub can_leave: bool,
}

impl Default for AutoJoinConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            guilds: Vec::new(),
            can_leave: true,
        }
    }
}
