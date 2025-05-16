// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{ChannelType, Snowflake, jwt::Claims};
use poem::{
	IntoResponse, Response, handler,
	http::StatusCode,
	web::{Data, Json, Path},
};
use sqlx::PgPool;
use util::{
	entities::{Channel, Recipient, User},
	errors::{ChannelError, Error},
};

#[handler]
pub async fn add_recipient(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Data(user): Data<&User>,
	Path((channel_id, user_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
	let mut channel = Channel::get_by_id(db, channel_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;
	channel.populate_relations(db).await?;

	if channel.channel_type.ne(&ChannelType::GroupDm) {
		let mut new_recipients =
			channel.recipients.as_ref().map_or(vec![], |v| v.iter().map(|r| r.id).collect());
		new_recipients.push(user.id);

		let new_dm_channel = Channel::create_dm_channel(db, new_recipients, user.id, None).await?;

		Ok(Json(new_dm_channel.into_inner()).into_response())
	} else {
		if let Some(recipients) = channel.recipients.as_ref() {
			if recipients.iter().any(|r| r.id == user_id) {
				return Err(Error::Channel(ChannelError::InvalidRecipient).into());
			}
		}

		let recipient = Recipient::create(db, channel_id, user_id).await?;

		// TODO: Emit events 'CHANNEL_CREATE' AND 'CHANNEL_RECIPIENT_ADD'

		Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
	}
}

#[handler]
pub async fn remove_recipient(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path((channel_id, user_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
	let mut channel = Channel::get_by_id(db, channel_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	if channel.channel_type.ne(&ChannelType::GroupDm)
		&& !(channel.owner_id.map(|u| u == claims.id).unwrap_or_default() || user_id == claims.id)
	{
		return Err(Error::Channel(ChannelError::InvalidRecipient).into()); // TODO: Should be a permissions error
	}

	let recipient = Recipient::get_by_channel_and_user_id(db, channel_id, user_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidRecipient))?;

	// TODO: Emit event?

	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}
