use chorus::types::{ChannelModifySchema, jwt::Claims, Snowflake};
use poem::{
    delete, get, handler, IntoResponse, post,
    put,
    Route, web::{Data, Json, Path},
};
use sqlx::MySqlPool;

use invites::{create_invite, get_invites};

use crate::{
    database::entities::Channel,
    errors::{ChannelError, Error},
};

mod invites;
mod messages;

pub fn setup_routes() -> Route {
    Route::new()
        .at(
            "/:channel_id",
            get(get_channel)
                .delete(delete_channel)
                .patch(modify_channel),
        )
        .at("/:channel_id/invites", get(get_invites).post(create_invite))
        .at(
            "/:channel_id/messages",
            get(messages::get_messages).post(messages::create_message),
        )
        .at(
            "/:channel_id/messages/bulk_delete",
            post(messages::bulk_delete::bulk_delete),
        )
        .at(
            "/:channel_id/messages/:message_id",
            get(messages::id::get_message)
                .delete(messages::id::delete_message)
                .patch(messages::id::edit_message),
        )
        .at(
            "/:channel_id/messages/:message_id/ack",
            post(messages::id::ack::acknowledge_message),
        )
        .at(
            "/:channel_id/messages/:message_id/crosspost",
            get(messages::id::crosspost::create_crosspost_message),
        )
        .at(
            "/:channel_id/messages/:message_id/reactions",
            delete(messages::id::reactions::delete_all_reactions),
        )
        .at(
            "/:channel_id/messages/:message_id/reactions/:emoji",
            get(messages::id::reactions::get_reaction)
                .delete(messages::id::reactions::delete_reaction),
        )
        .at(
            "/:channel_id/messages/:message_id/reactions/:emoji/:user_id",
            put(messages::id::reactions::add_reaction),
        )
}

#[handler]
pub async fn get_channel(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(channel_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
    let channel = Channel::get_by_id(db, channel_id)
        .await //?
        .expect("Failed to get channel data")
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
