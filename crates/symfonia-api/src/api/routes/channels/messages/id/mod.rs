// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{MessageModifySchema, Rights, Snowflake, jwt::Claims};
use poem::{
	IntoResponse, Response, handler,
	http::StatusCode,
	web::{Data, Json, Path},
};
use sqlx::PgPool;
use util::{
	entities::{Channel, Config, Message, User},
	errors::{ChannelError, Error},
};

pub(crate) mod ack;
pub(crate) mod crosspost;
pub(crate) mod reactions;

#[handler]
pub async fn edit_message(
	Data(db): Data<&PgPool>,
	Data(_claims): Data<&Claims>,
	Data(_config): Data<&Config>,
	Data(authed_user): Data<&User>,
	Path((channel_id, message_id)): Path<(Snowflake, Snowflake)>,
	Json(payload): Json<MessageModifySchema>,
) -> poem::Result<impl IntoResponse> {
	let mut message = Message::get_by_id(db, channel_id, message_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidMessage))?;

	// TODO: Check if the user has permission to edit the message
	if message.author_id != authed_user.id {
		if !authed_user.rights.has(Rights::MANAGE_MESSAGES, true) {
			// TODO: Check if user is instance admin
		}
	} else if !authed_user.rights.has(Rights::SELF_EDIT_MESSAGES, false) {
		return Err(Error::Channel(ChannelError::InvalidMessage))?;
	}

	message.modify(db, payload).await?;

	// TODO: Emit events

	Ok(Json(message))
}

#[handler]
pub async fn get_message(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	// Data(authed_user): Data<&User>,
	Path((channel_id, message_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
	let message = Message::get_by_id(db, channel_id, message_id)
		.await
		.expect("Failed to get message data")
		.ok_or(Error::Channel(ChannelError::InvalidMessage))?;

	if message.author_id != claims.id
	/* TODO: && permissions check 'READ_MESSAGE_HISTORY' */
	{
		return Err(Error::Channel(ChannelError::InvalidMessage))?;
	}

	Ok(Json(message))
}

#[handler]
pub async fn delete_message(
	Data(db): Data<&PgPool>,
	Data(_claims): Data<&Claims>,
	Data(authed_user): Data<&User>,
	Path((channel_id, message_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
	let channel = Channel::get_by_id(db, channel_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	let message = Message::get_by_id(db, channel.id, message_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidMessage))?;

	if message.author_id != authed_user.id && !authed_user.rights.has(Rights::MANAGE_MESSAGES, true)
	{
		// TODO: Check permissions on channel
	} else if !authed_user.rights.has(Rights::SELF_DELETE_MESSAGES, false) {
		return Err(Error::Channel(ChannelError::InvalidMessage))?; // TODO: Maybe a different error?
	}

	message.delete(db).await?;

	// TODO: Emit event 'MESSAGE_DELETE'

	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}
