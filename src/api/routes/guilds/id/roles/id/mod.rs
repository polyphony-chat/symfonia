use chorus::types::{jwt::Claims, PermissionFlags, RoleCreateModifySchema, Snowflake};
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json, Path},
    IntoResponse, Response,
};
use sqlx::MySqlPool;

use crate::{
    database::entities::Guild,
    errors::{Error, GuildError},
};

pub(crate) mod member_ids;
pub(crate) mod members;

#[handler]
pub async fn get_role(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path((guild_id, role_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    if !guild.has_member(db, claims.id).await? {
        return Err(Error::Guild(GuildError::MemberNotFound).into());
    }

    let role = guild
        .get_role(db, role_id)
        .await?
        .ok_or(Error::Guild(GuildError::RoleNotFound))?;

    Ok(Json(role))
}

#[handler]
pub async fn delete_role(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path((guild_id, role_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    let authed_member = guild
        .get_member(db, claims.id)
        .await?
        .ok_or(Error::Guild(GuildError::MemberNotFound))?;

    if authed_member
        .permissions
        .has_permission(PermissionFlags::MANAGE_ROLES)
    {
        return Err(Error::Guild(GuildError::InsufficientPermissions).into());
    }

    let role = guild
        .get_role(db, role_id)
        .await?
        .ok_or(Error::Guild(GuildError::RoleNotFound))?;

    role.delete(db).await?;

    // TODO: Emit event 'GUILD_ROLE_DELETE'

    Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}

#[handler]
pub async fn modify_role(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path((guild_id, role_id)): Path<(Snowflake, Snowflake)>,
    Json(payload): Json<RoleCreateModifySchema>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    let authed_member = guild
        .get_member(db, claims.id)
        .await?
        .ok_or(Error::Guild(GuildError::MemberNotFound))?;

    if !authed_member
        .permissions
        .has_permission(PermissionFlags::MANAGE_ROLES)
    {
        return Err(Error::Guild(GuildError::InsufficientPermissions).into());
    }

    let mut role = guild
        .get_role(db, role_id)
        .await?
        .ok_or(Error::Guild(GuildError::RoleNotFound))?;

    if let Some(name) = payload.name {
        role.name = name;
    }

    if let Some(color) = payload.color {
        role.color = color;
    }

    if let Some(hoist) = payload.hoist {
        role.hoist = hoist;
    }

    if let Some(mentionable) = payload.mentionable {
        role.mentionable = mentionable;
    }

    if let Some(permissions) = payload.permissions {
        role.permissions &= permissions;
    }

    if let Some(position) = payload.position {
        role.position = position as u16;
    }

    if let Some(icon) = payload.icon {
        // TODO: Handle icon in CDN
        // role.icon = Some(icon);
    }

    if let Some(unicode_emoji) = payload.unicode_emoji {
        if unicode_emoji.is_empty() {
            role.unicode_emoji = None;
        } else {
            role.unicode_emoji = Some(unicode_emoji);
        }
    }

    role.save(db).await?;

    // TODO: Emit event 'GUILD_ROLE_UPDATE'

    Ok(Json(role))
}
