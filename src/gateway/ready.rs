use std::collections::HashMap;

use chorus::types::{GatewayReady, Snowflake, UserNote};
use serde_json::json;
use sqlx::PgPool;

use crate::database::entities::{Channel, Guild, Note, Relationship};
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

    let notes_vec: Vec<UserNote> = Note::get_by_author_id(user_id, db)
        .await?
        .into_iter()
        .map(|x| x.into_inner())
        .collect();

    let mut notes = HashMap::new();
    for note in notes_vec.into_iter() {
        notes.insert(note.target_id, note.content);
    }

    // TODO: The session ID needs to be stored in the database and also removed on
    // session disconnect. This is a temporary solution.
    let session_id = Snowflake::generate().to_string();

    // TODO: There are a lot of missing fields here. Ideally, all of the fields should be
    // populated with the correct data.
    let ready = GatewayReady {
        user: user.clone().to_inner(),
        guilds,
        session_id,
        user_settings: Some(user.settings.into_inner()),
        relationships,
        private_channels,
        notes,
        ..Default::default()
    };
    log::debug!(target: "symfonia::gateway::ready::create_ready", "Created READY json payload: {:#?}", json!(ready));
    Ok(ready)
}
