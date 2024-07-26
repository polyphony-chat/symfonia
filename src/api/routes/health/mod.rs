/* 
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use poem::http::StatusCode;
use poem::web::Data;
use poem::{get, Route};
use poem::{handler, IntoResponse};
use sqlx::MySqlPool;

#[handler]
pub async fn healthz(Data(db): Data<&MySqlPool>) -> poem::Result<impl IntoResponse> {
    if db.is_closed() {
        return Err(poem::Error::from_status(StatusCode::SERVICE_UNAVAILABLE));
    }

    Ok(StatusCode::OK)
}

pub fn setup_routes() -> Route {
    Route::new()
        .at("/healthz", get(healthz))
        .at("/readyz", get(healthz))
}
