// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use poem::{
	IntoResponse, handler,
	web::{Data, Json},
};
use serde_json::json;
use util::entities::{Config, Guild, GuildMember, Message, User};

#[handler]
pub async fn stats(
	Data(db): Data<&sqlx::PgPool>,
	Data(cfg): Data<&Config>,
) -> poem::Result<impl IntoResponse> {
	if !cfg.security.stats_world_readable {
		// TODO: Check requester rights
	}

	let users = User::count(db).await?;
	let guilds = Guild::count(db).await?;
	let messages = Message::count(db).await?;
	let members = GuildMember::count(db).await?;

	Ok(Json(json!({
		"counts": {
			"user": users,
			"guild": guilds,
			"message": messages,
			"members": members
		}
	})))
}
