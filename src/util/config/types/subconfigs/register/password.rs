use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordConfiguration {
    pub required: bool,
    pub min_length: u8,
    pub min_numbers: u8,
    pub min_upper_case: u8,
    pub min_symbols: u8,
}

impl Default for PasswordConfiguration {
    fn default() -> Self {
        Self {
            required: false,
            min_length: 8,
            min_numbers: 2,
            min_upper_case: 2,
            min_symbols: 0,
        }
    }
}
