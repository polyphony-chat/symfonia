// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::APIError;
use poem::{
	IntoResponse, handler,
	web::{Data, Json},
};
use serde_json::json;
use util::entities::Config;

#[handler]
pub async fn domain(
	Data(db): Data<&sqlx::PgPool>,
	Data(cfg): Data<&Config>,
) -> Result<impl IntoResponse, APIError> {
	let cdn = if let Ok(endpoint) = std::env::var("CDN") {
		endpoint
	} else if let Some(endpoint) = &cfg.cdn.endpoint_public {
		endpoint.to_owned()
	} else {
		"http://localhost:3002".to_string()
	};

	let gateway = if let Ok(endpoint) = std::env::var("GATEWAY") {
		endpoint
	} else if let Some(endpoint) = &cfg.gateway.endpoint_public {
		endpoint.to_owned()
	} else {
		"ws://localhost:3003".to_string()
	};

	let api = if let Ok(endpoint) = std::env::var("API") {
		endpoint
	} else if let Some(endpoint) = &cfg.api.endpoint_public {
		endpoint.to_owned()
	} else {
		"http://localhost:3001/api".to_string()
	};

	Ok(Json(json!({
		"cdn": cdn,
		"gateway": gateway,
		"defaultApiVersion": cfg.api.default_version,
		"apiEndpoint": api
	})))
}
