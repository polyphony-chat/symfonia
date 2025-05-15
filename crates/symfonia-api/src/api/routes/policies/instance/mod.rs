// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
mod domain;
mod limits;

pub use domain::*;
pub use limits::*;
use poem::{
	IntoResponse, Route, get, handler,
	web::{Data, Json},
};
use util::entities::Config;

#[handler]
pub async fn general_config(Data(cfg): Data<&Config>) -> impl IntoResponse {
	Json(serde_json::to_value(&cfg.general).unwrap())
}

pub fn setup_routes() -> Route {
	Route::new().at("/", get(general_config)).at("/limits", get(limits)).at("/domains", get(domain))
}
