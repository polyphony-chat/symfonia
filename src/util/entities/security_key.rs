use serde::{Deserialize, Serialize};

use crate::util::Snowflake;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecurityKey {
    pub id: String,
    pub user_id: String,
    pub key_id: String,
    pub public_key: String,
    pub counter: u64,
    pub name: String,
}

impl Default for SecurityKey {
    fn default() -> Self {
        Self {
            id: Snowflake::generate().to_string(),
            user_id: String::new(),
            key_id: String::new(),
            public_key: String::new(),
            counter: 0,
            name: String::new(),
        }
    }
}
