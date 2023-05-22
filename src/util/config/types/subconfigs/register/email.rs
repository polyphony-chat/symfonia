use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistrationEmailConfiguration {
    pub reuired: bool,
    pub allowlist: bool,
    pub blacklist: bool,
    pub domains: Vec<String>,
}
impl Default for RegistrationEmailConfiguration {
    fn default() -> Self {
        Self {
            reuired: false,
            allowlist: false,
            blacklist: true,
            domains: Vec::new(),
        }
    }
}
