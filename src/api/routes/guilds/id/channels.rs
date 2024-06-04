use chorus::types::jwt::Claims;
use chorus::types::{ChannelModifySchema, ChannelType, Snowflake};
use poem::web::{Data, Json, Path};
use poem::{handler, IntoResponse};
use reqwest::StatusCode;
use sqlx::MySqlPool;

use crate::database::entities::Channel;

#[handler]
pub async fn get_channels(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(guild_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
    let channels = Channel::get_by_guild_id(db, guild_id).await?;

    Ok(Json(
        channels
            .into_iter()
            .map(|c| c.to_inner())
            .collect::<Vec<_>>(),
    ))
}

#[handler]
pub async fn create_channel(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(guild_id): Path<Snowflake>,
    Json(payload): Json<ChannelModifySchema>,
) -> poem::Result<impl IntoResponse> {
    let channel = Channel::create(
        db,
        payload.channel_type.unwrap_or(ChannelType::GuildText),
        payload.name,
        payload.nsfw.unwrap_or_default(),
        Some(guild_id),
        payload.parent_id,
        false,
        false,
        false,
        false,
    )
    .await?;

    Ok(Json(channel.to_inner()).with_status(StatusCode::CREATED))
}
