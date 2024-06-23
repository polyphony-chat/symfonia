use chorus::types::{jwt::Claims, ModifyCurrentGuildMemberSchema, PermissionFlags, Snowflake};
use poem::{
    handler,
    IntoResponse,
    web::{Data, Json, Path},
};
use sqlx::MySqlPool;

use crate::{
    database::entities::Guild,
    errors::{Error, GuildError},
};

#[handler]
pub async fn change_nickname(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path((guild_id, member_id)): Path<(Snowflake, String)>,
    Json(payload): Json<ModifyCurrentGuildMemberSchema>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    let mut authed_member = guild
        .get_member(db, claims.id)
        .await?
        .ok_or(Error::Guild(GuildError::MemberNotFound))?;

    if member_id.eq("@me")
        && authed_member
            .permissions
            .has_permission(PermissionFlags::CHANGE_NICKNAME)
    {
        authed_member.nick = payload.nick;
    } else if authed_member
        .permissions
        .has_permission(PermissionFlags::MANAGE_NICKNAMES)
    {
        let snowflake = Snowflake(member_id.parse::<u64>().unwrap());
        authed_member = guild
            .get_member(db, snowflake)
            .await?
            .ok_or(Error::Guild(GuildError::MemberNotFound))?;
        authed_member.nick = payload.nick;
    } else {
        return Err(Error::Guild(GuildError::InsufficientPermissions).into());
    }
    authed_member.save(db).await?;

    Ok(Json(authed_member.into_inner()))
}
