use chorus::types::{
    GuildBanBulkCreateSchema, GuildBanCreateSchema, GuildBansQuery, GuildBansSearchQuery,
    jwt::Claims, Snowflake,
};
use poem::{
    handler,
    IntoResponse,
    Response, web::{Data, Json, Path, Query},
};
use reqwest::StatusCode;
use sqlx::MySqlPool;

use crate::{
    database::entities::{Guild, GuildBan},
    errors::{Error, GuildError},
};

#[handler]
pub async fn get_bans(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(guild_id): Path<Snowflake>,
    Query(query): Query<GuildBansQuery>,
) -> poem::Result<impl IntoResponse> {
    let mut bans =
        GuildBan::get_by_guild(db, guild_id, query.before, query.after, query.limit).await?;
    bans.retain(|b| b.user_id != b.executor_id);

    for ban in bans.iter_mut() {
        ban.populate_relations(db).await?;
    }

    Ok(Json(bans))
}

#[handler]
pub async fn get_banned_user(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path((guild_id, user_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    let mut ban = GuildBan::get_by_user(db, guild.id, user_id)
        .await?
        .ok_or(Error::Guild(GuildError::BanNotFound))?;

    if ban.user_id == ban.executor_id {
        // Not sure we even need to worry about this
        return Err(Error::Guild(GuildError::BanNotFound).into());
    }

    ban.populate_relations(db).await?;

    Ok(Json(ban.into_inner()))
}

#[handler]
pub async fn create_ban(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path((guild_id, user_id)): Path<(Snowflake, Snowflake)>,
    Json(payload): Json<GuildBanCreateSchema>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    if GuildBan::get_by_user(db, guild.id, user_id)
        .await?
        .is_some()
    {
        return Err(Error::Guild(GuildError::BanAlreadyExists).into());
    }

    // TODO: Check permissions, and guild owner status

    GuildBan::create(db, guild.id, user_id, claims.id, None).await?; // TODO: Get reason from 'X-Audit-Log-Reason' header

    // TODO: Emit events 'GUILD_BAN_ADD' and optionally 'GUILD_MEMBER_REMOVE'

    Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}

#[handler]
pub async fn bulk_ban(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(guild_id): Path<Snowflake>,
    Json(payload): Json<GuildBanBulkCreateSchema>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    // TODO: Check permissions, and guild owner status

    let bans = GuildBan::builk_create(db, guild.id, payload.user_ids, claims.id, None).await?; // TODO: Get reason from 'X-Audit-Log

    // TODO: Emit events 'GUILD_BAN_ADD' and optionally 'GUILD_MEMBER_REMOVE'

    // TODO: This should return a json with banned_users and failed_users
    Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}

#[handler]
pub async fn search(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(guild_id): Path<Snowflake>,
    Query(query): Query<GuildBansSearchQuery>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    // TODO: Check permissions

    let mut bans =
        GuildBan::find_by_username(db, guild.id, &query.query, query.limit.unwrap_or(10)).await?;
    bans.retain(|b| b.user_id != b.executor_id);

    for ban in bans.iter_mut() {
        ban.populate_relations(db).await?;
    }

    Ok(Json(bans))
}

#[handler]
pub async fn delete_ban(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path((guild_id, user_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    // TODO: Check permissions, and guild owner status

    let ban = GuildBan::get_by_user(db, guild.id, user_id)
        .await?
        .ok_or(Error::Guild(GuildError::BanNotFound))?;
    // TODO: Get public user for event emit

    ban.delete(db).await?;

    // TODO: Emit event 'GUILD_BAN_REMOVE'

    Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}
