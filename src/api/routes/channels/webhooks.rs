use chorus::types::{CreateWebhookSchema, Snowflake, WebhookType};
use poem::{
    handler,
    IntoResponse,
    web::{Data, Json, Path},
};
use sqlx::MySqlPool;

use crate::{
    database::entities::{Channel, Config, User, Webhook},
    errors::{ChannelError, Error},
};

#[handler]
pub async fn get_webhooks(
    Data(db): Data<&MySqlPool>,
    Data(config): Data<&Config>,
    Data(user): Data<&User>,
    Path(channel_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
    // TODO: Check permissions 'MANAGE_WEBHOOKS'

    let channel = Channel::get_by_id(db, channel_id)
        .await?
        .ok_or(Error::Channel(ChannelError::InvalidChannel))?;
    if !channel.is_text() || channel.guild_id.is_none() {
        return Err(Error::Channel(ChannelError::InvalidChannelType).into());
    }

    let webhooks = Webhook::get_by_channel_id(db, channel_id).await?;
    Ok(Json(webhooks))
}

#[handler]
pub async fn create_webhook(
    Data(db): Data<&MySqlPool>,
    Data(config): Data<&Config>,
    Data(user): Data<&User>,
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
    if webhook_count > config.limits.channel.max_webhooks as i32 {
        return Err(Error::Channel(ChannelError::MaxWebhooksReached).into());
    }

    // TODO: Handle avatar_url

    let hook = Webhook::create(
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

    Ok(Json(hook))
}
