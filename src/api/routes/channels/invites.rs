use chorus::types::jwt::Claims;
use chorus::types::{CreateChannelInviteSchema, Snowflake};
use poem::web::{Data, Json, Path};
use poem::{handler, IntoResponse};
use sqlx::MySqlPool;

use crate::database::entities::Channel;
use crate::errors::{ChannelError, Error};

#[handler]
pub async fn create_invite(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(channel_id): Path<Snowflake>,
    Json(payload): Json<CreateChannelInviteSchema>,
) -> poem::Result<impl IntoResponse> {
    let channel = Channel::get_by_id(db, channel_id)
        .await?
        .ok_or(Error::Channel(ChannelError::InvalidChannel))?;

    // TODO: Check if the user has permission to create an invite
    // TODO: Check if the channel is a Group DM, and handle recipients
    // TODO: Check if inviter should be anonymous
    let invite = channel.create_invite(db, payload, None).await?;

    Ok(Json(invite.into_inner()))
}

#[handler]
pub async fn get_invites(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(channel_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
    let channel = Channel::get_by_id(db, channel_id)
        .await?
        .ok_or(Error::Channel(ChannelError::InvalidChannel))?;

    // TODO: Check if the user is allowed to see the invites of this channel

    let invites = channel.get_invites(db).await?;

    Ok(Json(invites))
}
