use serde::{Deserialize, Serialize};

use crate::util::config::types::subconfigs::limits::ratelimits::{
    auth::AuthRateLimit, RateLimitOptions,
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteRateLimit {
    pub guild: RateLimitOptions,
    pub webhook: RateLimitOptions,
    pub channel: RateLimitOptions,
    pub auth: AuthRateLimit,
}

impl Default for RouteRateLimit {
    fn default() -> Self {
        Self {
            guild: RateLimitOptions {
                bot: None,
                count: 5,
                window: 5,
                only_ip: false,
            },
            webhook: RateLimitOptions {
                bot: None,
                count: 10,
                window: 5,
                only_ip: false,
            },
            channel: RateLimitOptions {
                bot: None,
                count: 10,
                window: 5,
                only_ip: false,
            },
            auth: AuthRateLimit::default(),
        }
    }
}
