use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApiConfiguration {
    pub default_version: String,
    pub active_versions: Vec<String>,
    pub endpoint_public: Option<String>,
}

impl Default for ApiConfiguration {
    fn default() -> Self {
        Self {
            default_version: String::from("9"),
            active_versions: vec![
                String::from("6"),
                String::from("7"),
                String::from("8"),
                String::from("9"),
            ],
            endpoint_public: None,
        }
    }
}
