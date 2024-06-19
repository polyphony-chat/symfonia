use chorus::types::{jwt::Claims, Snowflake};
use poem::{
    handler,
    IntoResponse,
    web::{Data, Json, Path},
};
use serde_json::Number;
use sqlx::MySqlPool;

use crate::{
    database::entities::{Channel, Guild, GuildMember, Role},
    errors::{Error, GuildError},
};

pub mod channels;
pub(crate) mod invites;

#[handler]
pub async fn get_guild(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(guild_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
    let mut guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    let channels = Channel::get_by_guild_id(db, guild_id).await?;

    guild.channels = Some(channels.into_iter().map(|c| c.to_inner()).collect());

    let roles = Role::get_by_guild_id(db, guild_id).await?;

    guild.roles = Some(roles.into_iter().map(|r| r.to_inner()).collect());

    let member = GuildMember::get_by_id(db, claims.id, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::MemberNotFound))?;

    let mut object = serde_json::to_value(&guild).unwrap();
    object.as_object_mut().unwrap().insert(
        String::from("joined_at"),
        serde_json::Value::Number(Number::from(member.joined_at.timestamp())),
    );

    Ok(Json(object))
}
