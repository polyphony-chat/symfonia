// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{
	GetChannelMessagesSchema, MessageSendSchema, MessageType, Rights, Snowflake, jwt::Claims,
	types::guild_configuration::GuildFeatures,
};
use poem::{
	IntoResponse, handler,
	web::{Data, Json, Path, Query},
};
use sqlx::PgPool;
use util::{
	entities::{Channel, Config, Guild, Message, User},
	errors::{ChannelError, Error, GuildError, RateLimitError, UserError},
};

pub mod bulk_delete;
pub(crate) mod id;

#[handler]
pub async fn get_messages(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(channel_id): Path<Snowflake>,
	Query(payload): Query<GetChannelMessagesSchema>,
) -> poem::Result<impl IntoResponse> {
	let channel = Channel::get_by_id(db, channel_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	if !channel.is_text() {
		return Err(Error::Channel(ChannelError::InvalidChannelType).into());
	}

	// TODO: Check if the user has permission to view the channel (VIEW_CHANNEL)
	// TODO: Check if the user has permission to read previous messages
	// (READ_MESSAGE_HISTORY)

	let limit = payload.limit.unwrap_or(50);
	let mut messages = channel.get_messages(db, payload.anchor, limit).await?;

	messages.iter_mut().for_each(|message| {
		if let Some(reactions) = message.reactions.as_mut() {
			reactions.iter_mut().for_each(|reaction| {
				reaction.me = reaction.user_ids.contains(&claims.id);
			});
		}

		if let Some(attachments) = message.attachments.as_mut() {
			// TODO: Dynamically update proxy url in case the endpoint changed
			/*attachments.iter_mut().for_each(|attachment| {
				attachment.proxy_url = ;
			});*/
		}
	});

	Ok(Json(messages))
}

#[handler]
pub async fn create_message(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Data(config): Data<&Config>,
	Path(channel_id): Path<Snowflake>,
	Json(mut payload): Json<MessageSendSchema>,
) -> poem::Result<impl IntoResponse> {
	let mut channel = Channel::get_by_id(db, channel_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	if !channel.is_writeable() {
		return Err(Error::Channel(ChannelError::InvalidChannelType).into());
	}

	let user = User::get_by_id(db, claims.id).await?.ok_or(Error::User(UserError::InvalidUser))?;

	// TODO: Check if the user has permission to send messages in the channel
	// (SEND_MESSAGES)

	if let Some(nonce) = &payload.nonce {
		if let Some(existing) = Message::get_by_nonce(db, channel_id, claims.id, nonce).await? {
			return Ok(Json(existing));
		}
	}

	if !user.rights.has(Rights::BYPASS_RATE_LIMITS, true)
		&& config.limits.absolute_rate.send_message.enabled
	{
		let count = Message::count_by_user_in_window(
			db,
			channel_id,
			claims.id,
			config.limits.absolute_rate.send_message.window,
		)
		.await?;

		if count >= config.limits.absolute_rate.send_message.limit as i32 {
			// TODO: Include channel id?
			return Err(Error::RateLimit(RateLimitError::TooManyMessages).into());
		}
	}

	// TODO: Handle file uploads

	if payload
		.content
		.as_ref()
		.map(|c| c.len() as u32 > config.limits.message.max_characters)
		.unwrap_or_default()
	{
		return Err(Error::Channel(ChannelError::MessageTooLong).into());
	}

	// TODO: Handle stickers/activity

	if payload.content.as_ref().map(|c| c.is_empty()).unwrap_or_default()
		&& payload.embeds.as_ref().map(|e| e.is_empty()).unwrap_or_default()
		&& payload.attachments.as_ref().map(|a| a.is_empty()).unwrap_or_default()
		&& payload.sticker_ids.as_ref().map(|s| s.is_empty()).unwrap_or_default()
	{
		return Err(Error::Channel(ChannelError::EmptyMessage).into());
	}

	if let Some(reference) = payload.message_reference.as_ref() {
		// TODO: Check READ_MESSAGE_HISTORY
		if let Some(guild_id) = reference.guild_id {
			let guild = Guild::get_by_id(db, guild_id)
				.await?
				.ok_or(Error::Guild(GuildError::InvalidGuild))?;

			if !guild.features.contains(&GuildFeatures::CrossChannelReplies) {
				if channel.guild_id.map(|id| id != guild_id).unwrap_or(false) {
					// TODO: Throw bad message reference error
				}
				if reference.channel_id != channel.id {
					// TODO: Throw bad message reference error
				}
			}
		}
		payload.message_type = Some(MessageType::Reply);
	}

	let message = channel.create_message(db, payload, claims.id).await?;

	Ok(Json(message))
}

#[handler]
pub async fn create_greet_message(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Data(config): Data<&Config>,
	Path(channel_id): Path<Snowflake>,
	// Json(payload): Json<MessageSendSchema>, GreetSchema
) -> poem::Result<impl IntoResponse> {
	unimplemented!();
	Ok("")
}
