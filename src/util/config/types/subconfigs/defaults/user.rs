use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDefaults {
    pub premium: bool,
    pub premium_type: u8,
    pub verified: bool,
}

impl Default for UserDefaults {
    fn default() -> Self {
        Self {
            premium: true,
            premium_type: 2,
            verified: true,
        }
    }
}
