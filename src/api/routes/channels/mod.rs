use crate::database::entities::Channel;
use crate::errors::{ChannelError, Error};
use chorus::types::jwt::Claims;
use chorus::types::Snowflake;
use poem::web::{Data, Json, Path};
use poem::{get, handler, IntoResponse, Route};
use sqlx::MySqlPool;

pub fn setup_routes() -> Route {
    Route::new().at("/:channel_id", get(get_channel).delete(delete_channel))
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
