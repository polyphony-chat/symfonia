use std::env;

use poem::web::Json;
use poem::{handler, IntoResponse, Route};
use serde::Serialize;

pub fn setup_routes() -> Route {
    Route::new().at("/version", get_version)
}

#[handler]
pub async fn get_version() -> poem::Result<impl IntoResponse> {
    let version = Version {
        server: "symfonia".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    Ok(Json(version))
}

#[derive(Serialize)]
struct Version {
    server: String,
    version: String,
}
