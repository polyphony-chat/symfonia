use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailJetConfiguration {
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
}
