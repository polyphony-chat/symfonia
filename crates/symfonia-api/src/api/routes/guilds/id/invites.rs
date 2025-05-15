// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{GetInvitesSchema, Snowflake, jwt::Claims};
use poem::{
	IntoResponse, handler,
	web::{Data, Json, Path, Query},
};
use sqlx::PgPool;
use util::{
	entities::Guild,
	errors::{Error, GuildError},
};

#[handler]
pub async fn get_invites(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(guild_id): Path<Snowflake>,
	Query(query): Query<GetInvitesSchema>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	// TODO: Check if the user is allowed to see the invites of this guild

	let mut invites = guild.get_invites(db).await?;

	if query.with_counts.unwrap_or_default() {
		// TODO: Get approximate member count
		// TODO: Get approximate online member count (presences)

		for invite in invites.iter_mut() {
			invite.populate_relations(db).await?;
		}
	}

	Ok(Json(invites))
}
