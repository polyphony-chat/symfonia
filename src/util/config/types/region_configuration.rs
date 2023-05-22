use serde::{Deserialize, Serialize};

use crate::util::config::types::subconfigs::region::Region;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionConfiguration {
    pub default: String,
    pub use_default_as_optimal: bool,
    pub available: Vec<Region>,
}

impl Default for RegionConfiguration {
    fn default() -> Self {
        Self {
            default: String::from("spacebar"),
            use_default_as_optimal: true,
            available: vec![Region {
                id: String::from("spacebar"),
                name: String::from("spacebar"),
                endpoint: String::from("127.0.0.1:3004"),
                location: None,
                vip: false,
                custom: false,
                depreciated: false,
            }],
        }
    }
}
