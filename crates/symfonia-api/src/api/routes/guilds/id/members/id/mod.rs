// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{ModifyGuildMemberSchema, PermissionFlags, Rights, Snowflake, jwt::Claims};
use poem::{
	IntoResponse, Response, handler,
	http::StatusCode,
	web::{Data, Json, Path},
};
use sqlx::PgPool;
use util::{
	entities::{Guild, GuildMember, User},
	errors::{Error, GuildError, UserError},
};

pub(crate) mod nick;
pub(crate) mod roles;

#[handler]
pub async fn get_member(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path((guild_id, member_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	if !guild.has_member(db, claims.id).await? {
		return Err(Error::Guild(GuildError::MemberNotFound).into()); // TODO: Maybe these should just be 403/401s?
	}

	let mut member = GuildMember::get_by_id(db, guild.id, member_id)
		.await?
		.ok_or(Error::Guild(GuildError::MemberNotFound))?;

	member.populate_relations(db).await?;

	Ok(Json(member))
}

#[handler]
pub async fn modify_member(
	Data(db): Data<&PgPool>,
	Data(authed_user): Data<&User>,
	Path((guild_id, member_id)): Path<(Snowflake, String)>,
	Json(payload): Json<ModifyGuildMemberSchema>,
) -> poem::Result<impl IntoResponse> {
	let member_id = if member_id.eq("@me") {
		authed_user.id
	} else {
		Snowflake(member_id.parse::<u64>().map_err(|_| poem::http::StatusCode::BAD_REQUEST)?)
	};

	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	let Some(authed_member) = guild.get_member(db, authed_user.id).await? else {
		return Err(Error::Guild(GuildError::MemberNotFound).into());
	};

	let mut member =
		guild.get_member(db, member_id).await?.ok_or(Error::Guild(GuildError::MemberNotFound))?;

	if let Some(nick) = payload.nickname {
		if !authed_member.permissions.has_permission(PermissionFlags::MANAGE_NICKNAMES) {
			return Err(Error::Guild(GuildError::InsufficientPermissions).into());
		}

		if nick.is_empty() {
			member.nick = None;
		} else {
			member.nick = Some(nick);
		}
	}

	// bio and avatar?

	if let Some(flags) = payload.flags {
		member.flags = Some(flags);
	}

	if let Some(roles) = payload.roles {
		if !authed_member.permissions.has_permission(PermissionFlags::MANAGE_ROLES) {
			return Err(Error::Guild(GuildError::InsufficientPermissions).into());
		}

		member.roles.extend(roles);
		member.roles.dedup();
	}

	member.save(db).await?;

	// TODO: Emit event 'GUILD_MEMBER_UPDATE'

	Ok(Json(member.into_inner()))
}

#[handler]
pub async fn join_guild(
	Data(db): Data<&PgPool>,
	Data(authed_user): Data<&User>,
	Path((guild_id, member_id)): Path<(Snowflake, String)>,
) -> poem::Result<impl IntoResponse> {
	let member_id = if member_id.eq("@me") {
		if !authed_user.rights.has(Rights::JOIN_GUILDS, true) {
			return Err(Error::User(UserError::MissingRights(Rights::JOIN_GUILDS)).into());
		}
		authed_user.id
	} else {
		todo!("Handle OAuth2 scope");
		// Snowflake(member_id.parse::<u64>().map_err(|_| {
		//     poem::error::Error::from_string("Invalid member ID",
		// StatusCode::BAD_REQUEST) })?)
	};

	let mut guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	guild.populate_relations(db).await?;

	guild.add_member(db, member_id).await?;

	Ok(Json(guild.into_inner()))
}

#[handler]
pub async fn remove_member(
	Data(db): Data<&PgPool>,
	Data(authed_user): Data<&User>,
	Path((guild_id, member_id)): Path<(Snowflake, String)>,
) -> poem::Result<impl IntoResponse> {
	let member_id = if (member_id.eq("@me") || authed_user.id.to_string().eq(&member_id))
		&& authed_user.rights.has(Rights::SELF_LEAVE_GROUPS, false)
	{
		authed_user.id
	} else if authed_user.rights.has(Rights::KICK_BAN_MEMBERS, false) {
		Snowflake(member_id.parse::<u64>().unwrap())
	} else {
		todo!("Handle invalid rights")
	};

	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	let our_member = guild
		.get_member(db, authed_user.id)
		.await?
		.ok_or(Error::Guild(GuildError::MemberNotFound))?;
	if our_member.permissions.has_permission(PermissionFlags::KICK_MEMBERS) {
		return Err(Error::Guild(GuildError::InsufficientPermissions).into());
	}

	let member =
		guild.get_member(db, member_id).await?.ok_or(Error::Guild(GuildError::MemberNotFound))?;

	member.delete(db).await?;
	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}
