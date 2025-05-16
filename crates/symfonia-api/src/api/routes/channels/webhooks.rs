// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{CreateWebhookSchema, Snowflake, WebhookType};
use poem::{
	IntoResponse, handler,
	web::{Data, Json, Path},
};
use sqlx::PgPool;
use util::{
	entities::{Channel, Config, User, Webhook},
	errors::{ChannelError, Error},
};

#[handler]
pub async fn get_webhooks(
	Data(db): Data<&PgPool>,
	Path(channel_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
	// TODO: Check permissions 'MANAGE_WEBHOOKS'

	let channel = Channel::get_by_id(db, channel_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	let mut webhooks = Webhook::get_by_channel_id(db, channel.id).await?;

	for webhook in webhooks.iter_mut() {
		if let Some(user) = User::get_by_id(db, webhook.user_id).await? {
			webhook.user = Some(user.to_inner());
		}
	}

	Ok(Json(webhooks))
}

#[handler]
pub async fn create_webhook(
	Data(db): Data<&PgPool>,
	Data(user): Data<&User>,
	Data(config): Data<&Config>,
	Path(channel_id): Path<Snowflake>,
	Json(payload): Json<CreateWebhookSchema>,
) -> poem::Result<impl IntoResponse> {
	let channel = Channel::get_by_id(db, channel_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	if !channel.is_text() {
		return Err(Error::Channel(ChannelError::InvalidChannelType).into());
	}

	let Some(guild_id) = channel.guild_id else {
		return Err(Error::Channel(ChannelError::InvalidChannelType).into());
	};

	let webhook_count = Webhook::count_by_channel(db, channel_id).await?;

	if webhook_count >= config.limits.channel.max_webhooks as i32 {
		return Err(Error::Channel(ChannelError::MaxWebhooksReached).into());
	}

	// TODO: Reserved names?
	// TODO: Avatar file handling

	let mut hook = Webhook::create(
		db,
		&payload.name,
		guild_id,
		channel_id,
		user.id,
		payload.avatar,
		WebhookType::Incoming,
		None,
		None,
	)
	.await?;
	hook.user = Some(user.to_inner());

	Ok(Json(hook))
}
