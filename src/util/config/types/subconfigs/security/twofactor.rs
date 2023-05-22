use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoFactorConfiguration {
    pub generate_backup_codes: bool,
}

impl Default for TwoFactorConfiguration {
    fn default() -> Self {
        Self {
            generate_backup_codes: true,
        }
    }
}
