// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{ChannelModifySchema, Snowflake, jwt::Claims};
use invites::{create_invite, get_invites};
use poem::{
	IntoResponse, Route, delete, get, handler, post, put,
	web::{Data, Json, Path},
};
use sqlx::PgPool;
use util::{
	entities::Channel,
	errors::{ChannelError, Error},
};

mod followers;
mod invites;
mod messages;
mod permissions;
mod pins;
mod recipients;
mod typing;
mod webhooks;

pub fn setup_routes() -> Route {
	Route::new()
		.at("/:channel_id", get(get_channel).delete(delete_channel).patch(modify_channel))
		.at("/:channel_id/invites", get(get_invites).post(create_invite))
		.at("/:channel_id/messages", get(messages::get_messages).post(messages::create_message))
		.at("/:channel_id/messages/bulk_delete", post(messages::bulk_delete::bulk_delete))
		.at(
			"/:channel_id/messages/:message_id",
			get(messages::id::get_message)
				.delete(messages::id::delete_message)
				.patch(messages::id::edit_message),
		)
		.at("/:channel_id/messages/:message_id/ack", post(messages::id::ack::acknowledge_message))
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
		.at("/:channel_id/pins", get(pins::get_pinned_messages))
		.at(
			"/:channel_id/pins/:message_id",
			put(pins::add_pinned_message).delete(pins::remove_pinned_message),
		)
		.at("/:channel_id/webhooks", get(webhooks::get_webhooks).post(webhooks::create_webhook))
		.at("/:channel_id/followers", post(followers::create_following))
		.at(
			"/:channel_id/recipients",
			put(recipients::add_recipient).delete(recipients::remove_recipient),
		)
		.at(
			"/:channel_id/permissions/:overwrite_id",
			put(permissions::add_overwrite).delete(permissions::remove_overwrite),
		)
}

#[handler]
pub async fn get_channel(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(channel_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
	let channel = Channel::get_by_id(db, channel_id)
		.await //?
		.expect("Failed to get channel data")
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	// TODO: Check if the user has permission to read the channel

	Ok(Json(channel.into_inner()))
}

#[handler]
pub async fn delete_channel(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(channel_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
	let channel = Channel::get_by_id(db, channel_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	// TODO: Check if the user has permission to delete the channel
	// TODO: Check if the channel is a DM, and handle recipients
	channel.delete(db).await?;

	Ok(Json(channel.into_inner()))
}

#[handler]
pub async fn modify_channel(
	Data(db): Data<&PgPool>,
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

	Ok(Json(channel.into_inner()))
}
