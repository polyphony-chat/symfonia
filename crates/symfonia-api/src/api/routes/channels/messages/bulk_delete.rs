// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{Rights, Snowflake};
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

#[handler]
pub async fn bulk_delete(
	Data(db): Data<&PgPool>,
	Data(config): Data<&Config>,
	Data(user): Data<&User>,
	Path(channel_id): Path<Snowflake>,
	Json(ids): Json<Vec<Snowflake>>,
) -> poem::Result<impl IntoResponse> {
	// TODO: Make this bot only
	let channel = Channel::get_by_id(db, channel_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	if channel.guild_id.is_none() {
		// No bulk delete for DM channels
		return Err(Error::Channel(ChannelError::InvalidChannelType).into());
	}

	let superuser = user.rights.has(Rights::MANAGE_MESSAGES, true);
	let max_bulk_delete = config.limits.message.max_bulk_delete;
	if !superuser && ids.len() > max_bulk_delete as usize {
		return Err(Error::Channel(ChannelError::TooManyMessages(max_bulk_delete)).into());
	}

	// TODO: Check if the user has permission to delete the messages
	Message::bulk_delete(db, ids).await?;

	// TODO: Emit event 'MESSAGE_DELETE_BULK'

	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}
