use serde::{Deserialize, Serialize};

use crate::util::config::types::subconfigs::register::{
    DateOfBirthConfiguration, PasswordConfiguration, RegistrationEmailConfiguration,
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterConfiguration {
    pub email: RegistrationEmailConfiguration,
    pub date_of_birth: DateOfBirthConfiguration,
    pub password: PasswordConfiguration,
    pub disabled: bool,
    pub require_captcha: bool,
    pub require_invite: bool,
    pub guests_require_invite: bool,
    pub allow_new_registration: bool,
    pub allow_multiple_accounts: bool,
    pub block_proxies: bool,
    pub incrementing_discriminators: bool,
    pub default_rights: String,
}

impl Default for RegisterConfiguration {
    fn default() -> Self {
        Self {
            email: RegistrationEmailConfiguration::default(),
            date_of_birth: DateOfBirthConfiguration::default(),
            password: PasswordConfiguration::default(),
            disabled: false,
            require_captcha: true,
            require_invite: false,
            guests_require_invite: true,
            allow_new_registration: true,
            allow_multiple_accounts: true,
            block_proxies: true,
            incrementing_discriminators: false,
            default_rights: String::from("875069521787904"),
        }
    }
}
