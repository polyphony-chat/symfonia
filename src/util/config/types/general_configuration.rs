use serde::{Deserialize, Serialize};

use crate::util::Snowflake;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralConfiguration {
    pub instance_name: String,
    pub instance_description: String,
    pub front_page: Option<String>,
    pub tos_page: Option<String>,
    pub correspondence_email: Option<String>,
    pub correspondence_user_id: Option<String>,
    pub image: Option<String>,
    pub instance_id: Snowflake,
    pub auto_create_bot_users: bool,
}

impl Default for GeneralConfiguration {
    fn default() -> Self {
        Self {
            instance_name: String::from("Spacebar Instance"),
            instance_description: String::from(
                "This is a Spacebar instance made in the pre-release days",
            ),
            front_page: None,
            tos_page: None,
            correspondence_email: None,
            correspondence_user_id: None,
            image: None,
            instance_id: Snowflake::generate(),
            auto_create_bot_users: false,
        }
    }
}
