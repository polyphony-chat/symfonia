// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{PermissionFlags, Snowflake};
use poem::{
	IntoResponse, Response, handler,
	http::StatusCode,
	web::{Data, Path},
};
use sqlx::PgPool;
use util::{
	entities::{Guild, User},
	errors::{Error, GuildError},
};

#[handler]
pub async fn add_role(
	Data(db): Data<&PgPool>,
	Data(authed_user): Data<&User>,
	Path((guild_id, member_id, role_id)): Path<(Snowflake, Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	let authed_member = guild
		.get_member(db, authed_user.id)
		.await?
		.ok_or(Error::Guild(GuildError::MemberNotFound))?;

	if !authed_member.permissions.has_permission(PermissionFlags::MANAGE_ROLES) {
		return Err(Error::Guild(GuildError::InsufficientPermissions).into());
	}

	let mut member =
		guild.get_member(db, member_id).await?.ok_or(Error::Guild(GuildError::MemberNotFound))?;

	guild.get_role(db, role_id).await?.ok_or(Error::Guild(GuildError::InvalidRole))?;

	member.add_role(db, role_id).await?;

	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}

#[handler]
pub async fn remove_role(
	Data(db): Data<&PgPool>,
	Data(authed_user): Data<&User>,
	Path((guild_id, member_id, role_id)): Path<(Snowflake, Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	let authed_member = guild
		.get_member(db, authed_user.id)
		.await?
		.ok_or(Error::Guild(GuildError::MemberNotFound))?;

	if !authed_member.permissions.has_permission(PermissionFlags::MANAGE_ROLES) {
		return Err(Error::Guild(GuildError::InsufficientPermissions).into());
	}

	let mut member =
		guild.get_member(db, member_id).await?.ok_or(Error::Guild(GuildError::MemberNotFound))?;

	guild.get_role(db, role_id).await?.ok_or(Error::Guild(GuildError::InvalidRole))?;

	member.remove_role(db, role_id).await?;

	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}
