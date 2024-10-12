use chorus::types::{GatewayReady, Snowflake};
use sqlx::PgPool;

use crate::database::entities::{Channel, Guild, Relationship};
use crate::{database::entities::User, errors::Error};

pub async fn create_ready(user_id: Snowflake, db: &PgPool) -> Result<GatewayReady, Error> {
    let user = match User::get_by_id(db, user_id).await? {
        Some(uwuser) => uwuser,
        None => {
            return Err(Error::Custom(format!(
                "The user specified by user_id '{user_id}' does not exist in the database"
            )))
        }
    };
    let guild_ids = user.get_guild_ids(db).await?;
    let mut guilds = Vec::with_capacity(guild_ids.len());
    for guild_id in guild_ids.iter() {
        guilds.push(match Guild::get_by_id(db, *guild_id).await? {
            Some(guild) => guild.into_inner(),
            None => continue,
        });
    }

    let relationships = Relationship::get_all_by_id(user_id, db)
        .await?
        .into_iter()
        .map(|x| x.into_inner())
        .collect();

    let private_channels = Channel::get_private_of_user(user_id, db)
        .await?
        .into_iter()
        .map(|x| x.into_inner())
        .collect();

    let ready = GatewayReady {
        analytics_token: todo!(),
        auth_session_id_hash: todo!(),
        country_code: todo!(),
        api_version: todo!(),
        user: user.to_inner(),
        guilds,
        presences: todo!(),
        sessions: todo!(),
        session_id: todo!(),
        session_type: todo!(),
        resume_gateway_url: todo!(),
        shard: todo!(),
        user_settings: Some(*user.settings),
        user_settings_proto: todo!(),
        relationships,
        friend_suggestion_count: todo!(),
        private_channels,
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
