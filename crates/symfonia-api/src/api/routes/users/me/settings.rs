// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{UserSettings, jwt::Claims};
use poem::{
	IntoResponse, handler,
	web::{Data, Json},
};
use sqlx::PgPool;
use util::{
	entities::User,
	errors::{Error, UserError},
};

#[handler]
pub async fn get_settings(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
) -> poem::Result<impl IntoResponse> {
	let user = User::get_by_id(db, claims.id).await?.ok_or(Error::User(UserError::InvalidUser))?;

	Ok(Json(user.settings))
}

#[handler]
pub async fn update_settings(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Json(settings): Json<UserSettings>,
) -> poem::Result<impl IntoResponse> {
	let mut user =
		User::get_by_id(db, claims.id).await?.ok_or(Error::User(UserError::InvalidUser))?;

	user.settings = util::entities::UserSettings::consume(settings, user.settings_index.to_uint());
	// TODO: user.settings.update(db).await.map_err(Error::Sqlx)?;

	Ok(Json(user.settings))
}
