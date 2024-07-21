use chorus::types::{PermissionFlags, Snowflake, VoiceStateUpdateSchema};
use chrono::Utc;
use poem::{
    handler,
    http::StatusCode,
    IntoResponse,
    Response, web::{Data, Json, Path},
};
use sqlx::MySqlPool;

use crate::{
    database::entities::{Guild, User, VoiceState},
    errors::{Error, GuildError},
};

#[handler]
pub async fn update_voice_state(
    Data(db): Data<&MySqlPool>,
    Data(authed_user): Data<&User>,
    Path((guild_id, user_id)): Path<(Snowflake, String)>,
    Json(mut payload): Json<VoiceStateUpdateSchema>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    let user_id = if user_id.eq("@me") {
        authed_user.id
    } else {
        Snowflake(user_id.parse().unwrap())
    };

    let our_member = guild
        .get_member(db, user_id)
        .await?
        .ok_or(Error::Guild(GuildError::MemberNotFound))?;

    if payload.suppress.is_some()
        && user_id.ne(&authed_user.id)
        && !our_member
            .permissions
            .has_permission(PermissionFlags::MUTE_MEMBERS)
    {
        return Err(Error::Guild(GuildError::InsufficientPermissions).into());
    }

    if payload.suppress.is_none() {
        payload.request_to_speak_timestamp = Some(Utc::now());
    }
    if payload.request_to_speak_timestamp.is_some()
        && our_member
            .permissions
            .has_permission(PermissionFlags::REQUEST_TO_SPEAK)
    {
        return Err(Error::Guild(GuildError::InsufficientPermissions).into());
    }

    let mut voice_state =
        VoiceState::get_by_guild_and_channel(db, guild_id, payload.channel_id, user_id)
            .await?
            .ok_or(Error::Guild(GuildError::VoiceStateNotFound))?;

    if let Some(b) = payload.suppress {
        voice_state.suppress = b;
    }
    voice_state.request_to_speak_timestamp = payload.request_to_speak_timestamp;
    voice_state.save(db).await?;

    // TODO: Emit event 'VOICE_STATE_UPDATE'

    Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}
