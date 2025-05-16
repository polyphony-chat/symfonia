// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{Snowflake, jwt::Claims};
use poem::{
	IntoResponse, handler,
	web::{Data, Json, Path},
};
use serde_json::json;
use sqlx::PgPool;
use util::{
	entities::{Channel, ReadState},
	errors::{ChannelError, Error},
};

#[handler]
pub async fn acknowledge_message(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path((channel_id, message_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
	let _channel = Channel::get_by_id(db, channel_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	// TODO: Check if user can view channel (VIEW_CHANNEL)

	if let Some(mut read_state) =
		ReadState::get_by_user_and_channel(db, channel_id, claims.id).await?
	{
		read_state.last_message_id = Some(message_id);
		read_state.save(db).await?;
	} else {
		ReadState::create(db, channel_id, claims.id, Some(message_id)).await?;
	}

	// TODO: emit events
	Ok(Json(json!({"token": null})))
}
