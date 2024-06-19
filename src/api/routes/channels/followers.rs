use chorus::types::{
    AddFollowingChannelSchema, FollowedChannel, jwt::Claims, Snowflake, WebhookType,
};
use poem::{
    handler,
    IntoResponse,
    web::{Data, Json, Path},
};
use sqlx::MySqlPool;

use crate::{
    database::entities::{Channel, Config, Guild, Webhook},
    errors::{ChannelError, Error, GuildError},
};

#[handler]
pub async fn create_following(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Data(config): Data<&Config>,
    Path(channel_id): Path<Snowflake>,
    Json(payload): Json<AddFollowingChannelSchema>,
) -> poem::Result<impl IntoResponse> {
    let channel = Channel::get_by_id(db, channel_id)
        .await?
        .ok_or(Error::Channel(ChannelError::InvalidChannel))?;

    let target_channel = Channel::get_by_id(db, payload.webhook_channel_id)
        .await?
        .ok_or(Error::Channel(ChannelError::InvalidChannel))?;

    let Some(target_guild_id) = target_channel.guild_id else {
        return Err(Error::Channel(ChannelError::InvalidChannelType).into());
    };

    let Some(guild_id) = channel.guild_id else {
        return Err(Error::Channel(ChannelError::InvalidChannelType).into());
    };

    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    let webhook = Webhook::create(
        db,
        &format!(
            "{} #{}",
            guild.name.clone().unwrap_or_default(),
            channel.name.clone().unwrap_or_default(),
        ),
        target_guild_id,
        payload.webhook_channel_id,
        claims.id,
        None, // TODO: Make this the server icon
        WebhookType::ChannelFollower,
        Some(guild_id),
        None,
    )
    .await?;

    Ok(Json(FollowedChannel {
        channel_id,
        webhook_id: webhook.id,
    }))
}
