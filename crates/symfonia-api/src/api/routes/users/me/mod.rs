// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
mod settings;

use chorus::types::jwt::Claims;
use poem::{
	IntoResponse, Route, get, handler,
	web::{Data, Json},
};
use settings::{get_settings, update_settings};
use sqlx::PgPool;
use util::{
	entities::User,
	errors::{Error, UserError},
};

pub fn setup_routes() -> Route {
	Route::new().at("/", get(get_data)).at("/settings", get(get_settings).patch(update_settings))
}

#[handler]
pub async fn get_data(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
) -> poem::Result<impl IntoResponse> {
	let user = User::get_by_id(db, claims.id)
		.await
		.unwrap()
		.ok_or(Error::User(UserError::InvalidUser))
		.unwrap();

	Ok(Json(user))
}
