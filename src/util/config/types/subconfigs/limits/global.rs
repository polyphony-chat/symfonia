use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlobalRateLimit {
    pub limit: u16,
    pub window: u64,
    pub enabled: bool,
}

impl Default for GlobalRateLimit {
    fn default() -> Self {
        Self {
            limit: 100,
            window: 60 * 60 * 1000,
            enabled: true,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalRateLimits {
    pub register: GlobalRateLimit,
    pub send_message: GlobalRateLimit,
}

impl Default for GlobalRateLimits {
    fn default() -> Self {
        Self {
            register: GlobalRateLimit {
                limit: 25,
                ..Default::default()
            },
            send_message: GlobalRateLimit {
                limit: 200,
                window: 60 * 1000,
                ..Default::default()
            },
        }
    }
}
