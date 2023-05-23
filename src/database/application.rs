use crate::database::{Queryer, UserService};
use bitflags::Flags;
use polyphony_types::{
    entities::{Application, ApplicationFlags, User},
    errors::Error,
    utils::Snowflake,
};

pub struct ApplicationService;

impl ApplicationService {
    pub async fn create<'c, C: Queryer<'c>>(
        db: C,
        name: &str,
        summary: &str,
        owner_id: &Snowflake,
        verify_key: &str,
        flags: ApplicationFlags,
    ) -> Result<Application, Error> {
        let application = Application {
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
            .execute(db)
            .await?;

        Ok(application)
    }

    pub async fn get_by_id<'c, C: Queryer<'c>>(
        db: C,
        id: &Snowflake,
    ) -> Result<Option<Application>, Error> {
        sqlx::query_as("SELECT * FROM applications WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn get_by_owner<'c, C: Queryer<'c>>(
        db: C,
        owner_id: &Snowflake,
    ) -> Result<Vec<Application>, Error> {
        sqlx::query_as("SELECT * FROM applications WHERE owner_id = ?")
            .bind(owner_id)
            .fetch_all(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn get_owner<'c, C: Queryer<'c>>(
        application: &Application,
        db: C,
    ) -> Result<User, Error> {
        let u = UserService::get_by_id(db, &application.owner_id)
            .await?
            .unwrap(); // Unwrap the option since this should absolutely never fail
        Ok(u)
    }
}
