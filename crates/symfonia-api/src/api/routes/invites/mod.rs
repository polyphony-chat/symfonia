// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::jwt::Claims;
use poem::{
	IntoResponse, Route, get, handler,
	web::{Data, Json, Path},
};
use sqlx::PgPool;
use util::{
	entities::{Channel, Invite, User},
	errors::{ChannelError, Error, InviteError, UserError},
};

pub fn setup_routes() -> Route {
	Route::new().at("/:invite_code", get(get_invite).delete(delete_invite))
}
#[handler]
pub async fn get_invite(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(invite_code): Path<String>,
) -> poem::Result<impl IntoResponse> {
	let invite = Invite::get_by_code(db, &invite_code)
		.await?
		.ok_or(Error::Invite(InviteError::InvalidInvite))?;

	Ok(Json(invite.into_inner()))
}

#[handler]
pub async fn accept_invite(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(invite_code): Path<String>,
) -> poem::Result<impl IntoResponse> {
	let mut invite = Invite::get_by_code(db, &invite_code)
		.await?
		.ok_or(Error::Invite(InviteError::InvalidInvite))?;

	let user = User::get_by_id(db, claims.id).await?.ok_or(Error::User(UserError::InvalidUser))?;

	invite.join(db, &user).await?;

	Ok(Json(invite.into_inner()))
}

#[handler]
pub async fn delete_invite(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(invite_code): Path<String>,
) -> poem::Result<impl IntoResponse> {
	let mut invite = Invite::get_by_code(db, &invite_code)
		.await?
		.ok_or(Error::Invite(InviteError::InvalidInvite))?;

	if let Some(channel_id) = invite.channel_id {
		let channel = Channel::get_by_id(db, channel_id)
			.await?
			.ok_or(Error::Channel(ChannelError::InvalidChannel))?;
		// TODO: Check if the user has permission to delete an invite
		// TODO: Check if the channel is a Group DM, and handle recipients
		// TODO: Check if inviter should be anonymous
	} else {
		// TODO: Handle friend invites
	}

	invite.delete(db).await?;

	Ok(Json(invite.into_inner()))
}
