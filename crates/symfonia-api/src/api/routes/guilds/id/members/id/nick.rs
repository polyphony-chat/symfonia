// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{ModifyCurrentGuildMemberSchema, PermissionFlags, Snowflake, jwt::Claims};
use poem::{
	IntoResponse, handler,
	web::{Data, Json, Path},
};
use sqlx::PgPool;
use util::{
	entities::Guild,
	errors::{Error, GuildError},
};

#[handler]
pub async fn change_nickname(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path((guild_id, member_id)): Path<(Snowflake, String)>,
	Json(payload): Json<ModifyCurrentGuildMemberSchema>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	let mut authed_member =
		guild.get_member(db, claims.id).await?.ok_or(Error::Guild(GuildError::MemberNotFound))?;

	if member_id.eq("@me")
		&& authed_member.permissions.has_permission(PermissionFlags::CHANGE_NICKNAME)
	{
		authed_member.nick = payload.nickname;
	} else if authed_member.permissions.has_permission(PermissionFlags::MANAGE_NICKNAMES) {
		let snowflake = Snowflake(member_id.parse::<u64>().unwrap());
		authed_member = guild
			.get_member(db, snowflake)
			.await?
			.ok_or(Error::Guild(GuildError::MemberNotFound))?;
		authed_member.nick = payload.nickname;
	} else {
		return Err(Error::Guild(GuildError::InsufficientPermissions).into());
	}
	authed_member.save(db).await?;

	Ok(Json(authed_member.into_inner()))
}
