use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalTokensConfiguration {
    pub twitter: Option<String>,
}
