// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{MessageSendSchema, Snowflake, jwt::Claims};
use poem::{
	IntoResponse, handler,
	web::{Data, Json, Path},
};
use sqlx::PgPool;
use util::{
	entities::{Channel, Message, User},
	errors::{ChannelError, Error},
};

#[handler]
pub async fn create_crosspost_message(
	Data(db): Data<&PgPool>,
	Data(_claims): Data<&Claims>,
	Data(authed_user): Data<&User>,
	Path(channel_id): Path<Snowflake>,
	Json(payload): Json<MessageSendSchema>,
) -> poem::Result<impl IntoResponse> {
	let channel = Channel::get_by_id(db, channel_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	let Some(referenced) = &payload.message_reference else {
		return Err(Error::Channel(ChannelError::InvalidMessage).into()); // TODO: Maybe a generic bad request error?
	};

	let referenced_message = Message::get_by_id(db, referenced.channel_id, referenced.message_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidMessage))?;

	let message = Message::create(
		db,
		payload,
		channel.guild_id,
		referenced_message.channel_id,
		authed_user.id,
	)
	.await?;

	Ok(Json(message))
}
