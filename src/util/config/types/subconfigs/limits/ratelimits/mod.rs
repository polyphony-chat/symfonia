use serde::{Deserialize, Serialize};

pub mod auth;
pub mod route;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitOptions {
    pub bot: Option<u64>,
    pub count: u64,
    pub window: u64,
    pub only_ip: bool,
}
