use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailGunConfiguration {
    pub api_key: Option<String>,
    pub domain: Option<String>,
}
