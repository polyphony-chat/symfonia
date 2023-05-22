use serde::{Deserialize, Serialize};

use crate::util::config::types::subconfigs::limits::ratelimits::RateLimitOptions;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthRateLimit {
    pub login: RateLimitOptions,
    pub register: RateLimitOptions,
}

impl Default for AuthRateLimit {
    fn default() -> Self {
        Self {
            login: RateLimitOptions {
                bot: None,
                count: 5,
                window: 60,
                only_ip: false,
            },
            register: RateLimitOptions {
                bot: None,
                count: 2,
                window: 60 * 60 * 12,
                only_ip: false,
            },
        }
    }
}
