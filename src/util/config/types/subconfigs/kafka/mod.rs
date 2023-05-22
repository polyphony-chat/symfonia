use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KafkaBroker {
    pub ip: String,
    pub port: u16,
}
