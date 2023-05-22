use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateOfBirthConfiguration {
    pub required: bool,
    pub minimum: u8,
}

impl Default for DateOfBirthConfiguration {
    fn default() -> Self {
        Self {
            required: true,
            minimum: 13,
        }
    }
}
