use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SMTPConfiguration {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub secure: bool,
    pub username: Option<String>,
    pub password: Option<String>,
}
