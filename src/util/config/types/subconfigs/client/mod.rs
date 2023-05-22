use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientReleaseConfiguration {
    pub use_local_release: bool,
    pub upstream_version: String,
}

impl Default for ClientReleaseConfiguration {
    fn default() -> Self {
        Self {
            use_local_release: true,
            upstream_version: String::from("0.0.264"),
        }
    }
}
