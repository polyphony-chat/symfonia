// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use poem::{IntoResponse, Route, get, handler, http::StatusCode, web::Data};
use sqlx::PgPool;

#[handler]
pub async fn healthz(Data(db): Data<&PgPool>) -> poem::Result<impl IntoResponse> {
	if db.is_closed() {
		return Err(poem::Error::from_status(StatusCode::SERVICE_UNAVAILABLE));
	}

	Ok(StatusCode::OK)
}

pub fn setup_routes() -> Route {
	Route::new().at("/healthz", get(healthz)).at("/readyz", get(healthz))
}
