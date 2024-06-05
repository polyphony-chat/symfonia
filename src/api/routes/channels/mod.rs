use chorus::types::{ChannelModifySchema, Snowflake};
use chorus::types::jwt::Claims;
use poem::{get, handler, IntoResponse, post, Route};
use poem::web::{Data, Json, Path};
use sqlx::MySqlPool;

use invites::create_invite;

use crate::database::entities::Channel;
use crate::errors::{ChannelError, Error};

mod invites;

pub fn setup_routes() -> Route {
    Route::new()
        .at(
            "/:channel_id",
            get(get_channel)
                .delete(delete_channel)
                .patch(modify_channel),
        )
        .at("/:channel_id/invites", post(create_invite))
}

#[handler]
pub async fn get_channel(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(channel_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
    let channel = Channel::get_by_id(db, channel_id)
        .await?
        .ok_or(Error::Channel(ChannelError::InvalidChannel))?;

    // TODO: Check if the user has permission to read the channel

    Ok(Json(channel.to_inner()))
}

#[handler]
pub async fn delete_channel(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(channel_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
    let channel = Channel::get_by_id(db, channel_id)
        .await?
        .ok_or(Error::Channel(ChannelError::InvalidChannel))?;

    // TODO: Check if the user has permission to delete the channel
    // TODO: Check if the channel is a DM, and handle recipients
    channel.delete(db).await?;

    Ok(Json(channel.to_inner()))
}

#[handler]
pub async fn modify_channel(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(channel_id): Path<Snowflake>,
    Json(payload): Json<ChannelModifySchema>,
) -> poem::Result<impl IntoResponse> {
    let mut channel = Channel::get_by_id(db, channel_id)
        .await?
        .ok_or(Error::Channel(ChannelError::InvalidChannel))?;

    // TODO: Check if the user has permission to modify the channel

    channel.modify(payload);
    channel.save(db).await?;

    Ok(Json(channel.to_inner()))
}
