// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{PermissionOverwrite, PermissionOverwriteType, Snowflake, jwt::Claims};
use poem::{
	IntoResponse, Response, handler,
	http::StatusCode,
	web::{Data, Json, Path},
};
use sqlx::PgPool;
use util::{
	entities::{Channel, GuildMember, Role},
	errors::{ChannelError, Error, GuildError},
};

#[handler]
pub async fn add_overwrite(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path((channel_id, overwrite_id)): Path<(Snowflake, Snowflake)>,
	Json(payload): Json<PermissionOverwrite>,
) -> poem::Result<impl IntoResponse> {
	let mut channel = Channel::get_by_id(db, channel_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	let guild_id = channel.guild_id.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	// TODO: Check permissions

	if payload.overwrite_type.eq(&PermissionOverwriteType::Role) {
		if Role::get_by_id(db, overwrite_id).await?.is_none() {
			return Err(Error::Guild(GuildError::InvalidRole).into());
		}
	} else if payload.overwrite_type.eq(&PermissionOverwriteType::Member)
		&& GuildMember::get_by_id(db, overwrite_id, guild_id).await?.is_none()
	{
		return Err(Error::Guild(GuildError::MemberNotFound).into());
	}

	if let Some(overwrite) = channel
		.permission_overwrites
		.as_mut()
		.and_then(|x| x.iter_mut().find(|x| x.id == overwrite_id))
	{
		overwrite.allow &= payload.allow;
		overwrite.deny &= payload.deny;
	} else {
		channel.permission_overwrites = Some(sqlx::types::Json(vec![payload]));
	}
	channel.save(db).await?;

	// TODO: emit event 'CHANNEL_UPDATE'

	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}

#[handler]
pub async fn remove_overwrite(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path((channel_id, overwrite_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
	let mut channel = Channel::get_by_id(db, channel_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	let guild_id = channel.guild_id.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	// TODO: Check permissions

	if let Some(overwrites) = channel.permission_overwrites.as_mut() {
		overwrites.retain(|x| x.id != overwrite_id);
	}
	channel.save(db).await?;

	// TODO: emit event 'CHANNEL_UPDATE'

	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}
