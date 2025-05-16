// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{
	BulkGuildBanSchema, GetGuildBansQuery, GuildBanCreateSchema, GuildBansSearchQuery, Snowflake,
	jwt::Claims,
};
use poem::{
	IntoResponse, Response, handler,
	web::{Data, Json, Path, Query},
};
use reqwest::StatusCode;
use sqlx::PgPool;
use util::{
	entities::{Guild, GuildBan},
	errors::{Error, GuildError},
};

#[handler]
pub async fn get_bans(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(guild_id): Path<Snowflake>,
	Query(query): Query<GetGuildBansQuery>,
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
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path((guild_id, user_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

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
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path((guild_id, user_id)): Path<(Snowflake, Snowflake)>,
	Json(payload): Json<GuildBanCreateSchema>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	if GuildBan::get_by_user(db, guild.id, user_id).await?.is_some() {
		return Err(Error::Guild(GuildError::BanAlreadyExists).into());
	}

	// TODO: Check permissions, and guild owner status

	GuildBan::create(db, guild.id, user_id, claims.id, None).await?; // TODO: Get reason from 'X-Audit-Log-Reason' header

	// TODO: Emit events 'GUILD_BAN_ADD' and optionally 'GUILD_MEMBER_REMOVE'

	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}

#[handler]
pub async fn bulk_ban(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(guild_id): Path<Snowflake>,
	Json(payload): Json<BulkGuildBanSchema>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	// TODO: Check permissions, and guild owner status

	let bans = GuildBan::builk_create(db, guild.id, payload.user_ids, claims.id, None).await?; // TODO: Get reason from 'X-Audit-Log

	// TODO: Emit events 'GUILD_BAN_ADD' and optionally 'GUILD_MEMBER_REMOVE'

	// TODO: This should return a json with banned_users and failed_users
	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}

#[handler]
pub async fn search(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(guild_id): Path<Snowflake>,
	Query(query): Query<GuildBansSearchQuery>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	// TODO: Check permissions

	let mut bans =
		GuildBan::find_by_username(db, guild.id, &query.query, query.limit.unwrap_or(10).into())
			.await?;
	bans.retain(|b| b.user_id != b.executor_id);

	for ban in bans.iter_mut() {
		ban.populate_relations(db).await?;
	}

	Ok(Json(bans))
}

#[handler]
pub async fn delete_ban(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path((guild_id, user_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	// TODO: Check permissions, and guild owner status

	let ban = GuildBan::get_by_user(db, guild.id, user_id)
		.await?
		.ok_or(Error::Guild(GuildError::BanNotFound))?;
	// TODO: Get public user for event emit

	ban.delete(db).await?;

	// TODO: Emit event 'GUILD_BAN_REMOVE'

	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}
