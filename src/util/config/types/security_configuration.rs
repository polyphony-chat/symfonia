use base64::Engine;
use serde::{Deserialize, Serialize};

use crate::util::config::types::subconfigs::security::{
    CaptchaConfiguration, TwoFactorConfiguration,
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityConfiguration {
    pub captcha: CaptchaConfiguration,
    pub two_factor: TwoFactorConfiguration,
    pub auto_update: bool,
    pub request_signature: String,
    pub jwt_secret: String,
    pub forwarded_for: Option<String>,
    pub ipdata_api_key: Option<String>,
    pub mfa_backup_code_count: u8,
    pub stats_world_readable: bool,
    pub default_registration_token_expiration: u64,
}

impl Default for SecurityConfiguration {
    fn default() -> Self {
        let mut req_sig: [u8; 32] = [0; 32];
        openssl::rand::rand_bytes(&mut req_sig);
        let mut jwt_secret: [u8; 256] = [0; 256];
        openssl::rand::rand_bytes(&mut jwt_secret);
        Self {
            captcha: Default::default(),
            two_factor: Default::default(),
            auto_update: true,
            request_signature: base64::engine::general_purpose::STANDARD.encode(req_sig),
            jwt_secret: base64::engine::general_purpose::STANDARD.encode(jwt_secret),
            forwarded_for: None,
            ipdata_api_key: Some(String::from(
                "eca677b284b3bac29eb72f5e496aa9047f26543605efe99ff2ce35c9",
            )),
            mfa_backup_code_count: 10,
            stats_world_readable: true,
            default_registration_token_expiration: 1000 * 60 * 60 * 24 * 7,
        }
    }
}
