use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelLimits {
    pub max_pins: u16,
    pub max_topic: u16,
    pub max_webhooks: u16,
}

impl Default for ChannelLimits {
    fn default() -> Self {
        Self {
            max_pins: 500,
            max_topic: 1024,
            max_webhooks: 100,
        }
    }
}
