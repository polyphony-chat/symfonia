// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{GuildGetMembersQuery, GuildMembersSearchQuery, Snowflake, jwt::Claims};
use poem::{
	IntoResponse, handler,
	web::{Data, Json, Path, Query},
};
use sqlx::PgPool;
use util::{
	entities::{Guild, GuildMember},
	errors::{Error, GuildError},
};

pub(crate) mod id;

#[handler]
pub async fn get_members(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(guild_id): Path<Snowflake>,
	Query(query): Query<GuildGetMembersQuery>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	if !guild.has_member(db, claims.id).await? {
		return Err(Error::Guild(GuildError::MemberNotFound).into());
	}

	let members =
		GuildMember::get_by_guild_id(db, guild_id, query.limit.unwrap_or(1), query.after).await?;

	Ok(Json(members))
}

// Not for user accounts, bot / internal only
#[handler]
pub async fn search_members(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(guild_id): Path<Snowflake>,
	Query(query): Query<GuildMembersSearchQuery>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	let member =
		guild.get_member(db, claims.id).await?.ok_or(Error::Guild(GuildError::MemberNotFound))?;

	// TODO: Check if the member is a bot

	let members = guild
		.search_members(db, &query.query, query.limit.map(|l| l.min(1000)).unwrap_or(1))
		.await?;

	Ok("")
}
