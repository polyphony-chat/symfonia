use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CdnConfiguration {
    pub resize_height_max: u64,
    pub resize_width_max: u64,
    pub imagor_server_url: Option<String>,

    pub endpoint_public: Option<String>,
    pub endpoint_private: Option<String>,
}

impl Default for CdnConfiguration {
    fn default() -> Self {
        Self {
            resize_height_max: 1000,
            resize_width_max: 1000,
            imagor_server_url: None,

            endpoint_private: None,
            endpoint_public: None,
        }
    }
}
