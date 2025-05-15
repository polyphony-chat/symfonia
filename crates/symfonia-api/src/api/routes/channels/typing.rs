// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{Snowflake, jwt::Claims};
use poem::{
	IntoResponse, Response, handler,
	web::{Data, Path},
};
use reqwest::StatusCode;
use sqlx::PgPool;
use util::{
	entities::{Channel, GuildMember},
	errors::{ChannelError, Error, GuildError},
};

#[handler]
pub async fn typing_indicator(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(channel_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
	let channel = Channel::get_by_id(db, channel_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	// TODO: Handle dm/group dm channels
	let Some(guild_id) = channel.guild_id else {
		return Err(Error::Channel(ChannelError::InvalidChannelType).into());
	};

	let member = GuildMember::get_by_id(db, claims.id, guild_id)
		.await?
		.ok_or(Error::Guild(GuildError::MemberNotFound))?;

	// TODO: Emit event 'TYPING_START'

	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}
