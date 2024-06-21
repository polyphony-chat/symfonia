use chorus::types::{EmojiCreateSchema, EmojiModifySchema, jwt::Claims, Snowflake};
use poem::{
    handler,
    IntoResponse,
    Response, web::{Data, Json, Path},
};
use reqwest::StatusCode;
use sqlx::MySqlPool;

use crate::{
    database::entities::{Config, Emoji, Guild},
    errors::{Error, GuildError},
};

#[handler]
pub async fn get_emojis(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(guild_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    if !guild.has_member(db, claims.id).await? {
        return Err(Error::Guild(GuildError::MemberNotFound).into());
    }

    let emojis = guild.get_emojis(db).await?;
    Ok(Json(emojis))
}

#[handler]
pub async fn get_emoji(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path((guild_id, emoji_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    if !guild.has_member(db, claims.id).await? {
        return Err(Error::Guild(GuildError::MemberNotFound).into());
    }

    let emoji = guild
        .get_emoji(db, emoji_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidEmoji))?;

    Ok(Json(emoji))
}

#[handler]
pub async fn create_emoji(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Data(config): Data<&Config>,
    Path(guild_id): Path<Snowflake>,
    Json(payload): Json<EmojiCreateSchema>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    if !guild.has_member(db, claims.id).await? {
        return Err(Error::Guild(GuildError::MemberNotFound).into());
    }

    let count = Emoji::count(db, guild.id).await?;
    if count >= config.limits.guild.max_emojis as i32 {
        return Err(Error::Guild(GuildError::MaxEmojisReached(
            config.limits.guild.max_emojis as i32,
        ))
        .into());
    }

    // TODO: Store the emoji on the CDN
    // TODO: Determine if the emoji is animated
    // TODO: Determine if the emoji should require colons

    let emoji_name = payload
        .name
        .unwrap_or_else(|| String::from("emoji_file_name"));

    let emoji = Emoji::create(
        db,
        guild.id,
        Some(claims.id),
        &emoji_name,
        false,
        false,
        false,
        payload.roles,
    )
    .await?;

    // TODO: Emit event 'GUILD_EMOJIS_UPDATE'

    Ok(Json(emoji).with_status(StatusCode::CREATED))
}

#[handler]
pub async fn modify_emoji(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path((guild_id, emoji_id)): Path<(Snowflake, Snowflake)>,
    Json(payload): Json<EmojiModifySchema>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    if !guild.has_member(db, claims.id).await? {
        return Err(Error::Guild(GuildError::MemberNotFound).into());
    }

    // TODO: Check permission 'MANAGE_EMOJIS_AND_STICKERS'

    let mut emoji = guild
        .get_emoji(db, emoji_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidEmoji))?;

    if payload.name.is_some() {
        emoji.name = payload.name;
    }
    if payload.roles.is_some() {
        emoji.roles = payload.roles.map(|v| sqlx::types::Json(v));
    }

    emoji.save(db).await?;

    // TODO: Emit event 'GUILD_EMOJIS_UPDATE'

    Ok(Json(emoji))
}

#[handler]
pub async fn delete_emoji(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path((guild_id, emoji_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    if !guild.has_member(db, claims.id).await? {
        return Err(Error::Guild(GuildError::MemberNotFound).into());
    }

    // TODO: Check permission 'MANAGE_EMOJIS_AND_STICKERS'

    let emoji = guild
        .get_emoji(db, emoji_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidEmoji))?;

    emoji.delete(db).await?;

    // TODO: Emit event 'GUILD_EMOJIS_UPDATE'

    Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}
