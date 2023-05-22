use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum GifProvider {
    #[default]
    Tenor,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifConfiguration {
    pub enabled: bool,
    pub provider: GifProvider,
    pub api_key: Option<String>,
}

impl Default for GifConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            provider: GifProvider::Tenor,
            api_key: Some(String::from("LIVDSRZULELA")),
        }
    }
}
