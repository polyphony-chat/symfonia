// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{Snowflake, jwt::Claims};
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
pub async fn get_member_ids(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path((guild_id, role_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	if !guild.has_member(db, claims.id).await? {
		return Err(Error::Guild(GuildError::MemberNotFound).into());
	}

	let member_ids =
		guild.get_members_by_role(db, role_id).await?.into_iter().map(|m| m.id).collect::<Vec<_>>();

	Ok(Json(member_ids))
}
