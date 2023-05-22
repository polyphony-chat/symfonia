use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginConfiguration {
    pub require_captcha: bool,
    pub require_verification: bool,
}
