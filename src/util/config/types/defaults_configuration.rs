use serde::{Deserialize, Serialize};

use crate::util::config::types::subconfigs::defaults::{guild::GuildDefaults, user::UserDefaults};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultsConfiguration {
    pub guild: GuildDefaults,
    pub user: UserDefaults,
}
