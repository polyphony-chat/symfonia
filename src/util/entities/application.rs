use crate::errors::Error;
use crate::util::entities::user::User;
use crate::util::Snowflake;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Decode, Encode, FromRow, Type};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Application {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub r#type: Option<sqlx::types::Json<Value>>,
    pub hook: bool,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub verify_key: String,
    pub owner_id: Snowflake,
    pub flags: u64,
    pub redirect_uris: Option<sqlx::types::Json<Vec<String>>>,
    pub rpc_application_state: i64,
    pub store_application_state: i64,
    pub verification_state: i64,
    pub interactions_endpoint_url: Option<String>,
    pub integration_public: bool,
    pub integration_require_code_grant: bool,
    pub discoverability_state: i64,
    pub discovery_eligibility_flags: i64,
    pub bot_user_id: Snowflake,
    pub tags: Option<sqlx::types::Json<Vec<String>>>,
    pub cover_image: Option<String>,
    pub install_params: Option<sqlx::types::Json<InstallParams>>,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub team_id: Option<Snowflake>,
}

impl Application {
    pub async fn create(
        conn: &mut sqlx::MySqlConnection,
        name: &str,
        summary: &str,
        owner_id: &Snowflake,
        verify_key: &str,
        flags: ApplicationFlags,
    ) -> Result<Self, Error> {
        let application = Self {
            id: Snowflake::generate(),
            name: name.to_string(),
            icon: None,
            description: None,
            summary: Some(summary.to_string()),
            r#type: None,
            hook: true,
            bot_public: true,
            bot_require_code_grant: false,
            verify_key: verify_key.to_string(),
            owner_id: owner_id.to_owned(),
            flags: flags.bits(),
            redirect_uris: None,
            rpc_application_state: 0,
            store_application_state: 0,
            verification_state: 0,
            interactions_endpoint_url: None,
            integration_public: true,
            integration_require_code_grant: false,
            discoverability_state: 1,
            discovery_eligibility_flags: 2240,
            bot_user_id: Snowflake::generate(), // TODO: replace with generated bot user
            tags: None,
            cover_image: None,
            install_params: None,
            terms_of_service_url: None,
            privacy_policy_url: None,
            team_id: None,
        };

        let _res = sqlx::query("INSERT INTO applications (id, name, summary, hook, bot_public, verify_key, owner_id, flags, integration_public, discoverability_state, discovery_eligibility_flags) VALUES (?, ?, ?, true, true, ?, ?, ?, true, 1, 2240)")
            .bind(&application.id)
            .bind(name)
            .bind(summary)
            .bind(verify_key)
            .bind(owner_id)
            .bind(flags.bits())
            .execute(conn)
            .await?;

        Ok(application)
    }

    pub async fn get_by_id(
        conn: &mut sqlx::MySqlConnection,
        id: &Snowflake,
    ) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM applications WHERE id = ?")
            .bind(id)
            .fetch_optional(conn)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn get_by_owner(
        conn: &mut sqlx::MySqlConnection,
        owner_id: &Snowflake,
    ) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * FROM applications WHERE owner_id = ?")
            .bind(owner_id)
            .fetch_all(conn)
            .await
            .map_err(Error::SQLX)
    }

    pub fn flags(&self) -> ApplicationFlags {
        ApplicationFlags::from_bits(self.flags.to_owned()).unwrap()
    }

    pub async fn get_owner(&self, conn: &mut sqlx::MySqlConnection) -> Result<User, Error> {
        let u = User::get_by_id(conn, &self.owner_id).await?.unwrap(); // Unwrap the option since this should absolutely never fail
        Ok(u)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallParams {
    pub scopes: Vec<String>,
    pub permissions: String,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub struct ApplicationFlags: u64 {
        const APPLICATION_AUTO_MODERATION_RULE_CREATE_BADGE = 1 << 6;
        const GATEWAY_PRESENCE = 1 << 12;
        const GATEWAY_PRESENCE_LIMITED = 1 << 13;
        const GATEWAY_GUILD_MEMBERS = 1 << 14;
        const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
        const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
        const EMBEDDED = 1 << 17;
        const GATEWAY_MESSAGE_CONTENT = 1 << 18;
        const GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19;
        const APPLICATION_COMMAND_BADGE = 1 << 23;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCommand {
    pub id: Snowflake,
    pub application_id: Snowflake,
    pub name: String,
    pub description: String,
    pub options: Vec<ApplicationCommandOption>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCommandOption {
    pub r#type: ApplicationCommandOptionType,
    pub name: String,
    pub description: String,
    pub required: bool,
    pub choices: Vec<ApplicationCommandOptionChoice>,
    pub options: Vec<ApplicationCommandOption>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCommandOptionChoice {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ApplicationCommandOptionType {
    #[serde(rename = "SUB_COMMAND")]
    SubCommand = 1,
    #[serde(rename = "SUB_COMMAND_GROUP")]
    SubCommandGroup = 2,
    #[serde(rename = "STRING")]
    String = 3,
    #[serde(rename = "INTEGER")]
    Integer = 4,
    #[serde(rename = "BOOLEAN")]
    Boolean = 5,
    #[serde(rename = "USER")]
    User = 6,
    #[serde(rename = "CHANNEL")]
    Channel = 7,
    #[serde(rename = "ROLE")]
    Role = 8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCommandInteractionData {
    pub id: Snowflake,
    pub name: String,
    pub options: Vec<ApplicationCommandInteractionDataOption>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCommandInteractionDataOption {
    pub name: String,
    pub value: Value,
    pub options: Vec<ApplicationCommandInteractionDataOption>,
}
