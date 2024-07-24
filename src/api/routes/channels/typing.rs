use chorus::types::{jwt::Claims, Snowflake};
use poem::{
    handler,
    web::{Data, Path},
    IntoResponse, Response,
};
use reqwest::StatusCode;
use sqlx::MySqlPool;

use crate::{
    database::entities::{Channel, GuildMember},
    errors::{ChannelError, Error, GuildError},
};

#[handler]
pub async fn typing_indicator(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(channel_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
    let channel = Channel::get_by_id(db, channel_id)
        .await?
        .ok_or(Error::Channel(ChannelError::InvalidChannel))?;

    // TODO: Handle dm/group dm channels
    let Some(guild_id) = channel.guild_id else {
        return Err(Error::Channel(ChannelError::InvalidChannelType).into());
    };

    let member = GuildMember::get_by_id(db, claims.id, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::MemberNotFound))?;

    // TODO: Emit event 'TYPING_START'

    Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}
