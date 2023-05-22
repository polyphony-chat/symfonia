use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLimits {
    pub max_guilds: u64,
    pub max_username: u16,
    pub max_friends: u64,
}

impl Default for UserLimits {
    fn default() -> Self {
        Self {
            max_guilds: 1048576,
            max_username: 32,
            max_friends: 5000,
        }
    }
}
