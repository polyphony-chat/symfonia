use chorus::types::{GatewayReady, Snowflake};
use sqlx::PgPool;

use crate::database::entities::User;
use crate::errors::Error;

pub async fn create_ready(user_id: Snowflake, db: &PgPool) -> Result<GatewayReady, Error> {
    let user = match User::get_by_id(db, user_id).await? {
        Some(uwuser) => uwuser,
        None => {
            return Err(Error::Custom(format!(
                "The user specified by user_id '{user_id}' does not exist in the database"
            )))
        }
    };
    let ready = GatewayReady {
        analytics_token: todo!(),
        auth_session_id_hash: todo!(),
        country_code: todo!(),
        api_version: todo!(),
        user: user.to_inner(),
        guilds: todo!(),
        presences: todo!(),
        sessions: todo!(),
        session_id: todo!(),
        session_type: todo!(),
        resume_gateway_url: todo!(),
        shard: todo!(),
        user_settings: Some(*user.settings),
        user_settings_proto: todo!(),
        relationships: todo!(),
        friend_suggestion_count: todo!(),
        private_channels: todo!(),
        notes: todo!(),
        merged_presences: todo!(),
        users: todo!(),
        auth_token: todo!(),
        authenticator_types: todo!(),
        required_action: todo!(),
        geo_ordered_rtc_regions: todo!(),
        tutorial: todo!(),
        api_code_version: todo!(),
        experiments: todo!(),
        guild_experiments: todo!(),
        _trace: todo!(),
    };
    todo!()
}
