use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointConfiguration {
    pub endpoint_client: Option<String>,
    pub endpoint_private: Option<String>,
    pub endpoint_public: Option<String>,
}
