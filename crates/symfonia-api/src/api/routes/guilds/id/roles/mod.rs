// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{PermissionFlags, RoleCreateModifySchema, RolePositionUpdateSchema, Snowflake};
use poem::{
	IntoResponse, handler,
	web::{Data, Json, Path},
};
use sqlx::PgPool;
use util::{
	SharedEventPublisherMap,
	entities::{Config, Guild, Role, User},
	errors::{Error, GuildError},
};

pub(crate) mod id;
pub(crate) mod member_counts;

#[handler]
pub async fn get_roles(
	Data(db): Data<&PgPool>,
	Data(authed_user): Data<&User>,
	Path(guild_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	if !guild.has_member(db, authed_user.id).await? {
		return Err(Error::Guild(GuildError::MemberNotFound).into());
	}

	let roles = guild.get_roles(db).await?;

	Ok(Json(roles))
}

#[handler]
pub async fn create_role(
	Data(db): Data<&PgPool>,
	Data(publisher_map): Data<&SharedEventPublisherMap>,
	Data(authed_user): Data<&User>,
	Data(config): Data<&Config>,
	Path(guild_id): Path<Snowflake>,
	Json(payload): Json<RoleCreateModifySchema>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	let role_count = guild.count_roles(db).await?;

	if config.limits.guild.max_roles as i32 >= role_count {
		return Err(
			Error::Guild(GuildError::RoleLimitReached(config.limits.guild.max_roles)).into()
		);
	}

	let name = payload.name.unwrap_or_else(|| format!("Role {}", role_count + 1));

	let role = Role::create(
		db,
		publisher_map.clone(),
		None,
		guild.id,
		&name,
		payload.color.unwrap_or(0.),
		payload.hoist.unwrap_or_default(),
		false,
		true,
		payload.permissions.unwrap_or_default(),
		1,
		None,
		None,
	)
	.await?;

	// TODO: Emit event 'GUILD_ROLE_CREATE'

	Ok(Json(role.into_inner()))
}

#[handler]
pub async fn update_position(
	Data(db): Data<&PgPool>,
	Data(authed_user): Data<&User>,
	Data(config): Data<&Config>,
	Path(guild_id): Path<Snowflake>,
	Json(payload): Json<RolePositionUpdateSchema>,
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

	let mut role =
		Role::get_by_id(db, payload.id).await?.ok_or(Error::Guild(GuildError::RoleNotFound))?;

	if role.guild_id != guild.id {
		return Err(Error::Guild(GuildError::RoleNotFound).into());
	}

	role.position = payload.position.into();
	role.save(db).await?;

	let mut roles = guild.get_roles(db).await?;
	roles.sort_by(|a, b| a.position.cmp(&b.position));

	Ok(Json(roles.into_iter().map(|r| r.into_inner()).collect::<Vec<_>>()))
}
