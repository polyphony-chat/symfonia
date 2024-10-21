use std::env;

use chorus::types::types::domains_configuration::WellKnownResponse;
use poem::get;
use poem::{handler, web::Json, IntoResponse, Route};

use crate::api::BIND_API;

pub fn setup_routes() -> Route {
    Route::new().at("/spacebar", get(get_well_known))
}

#[handler]
pub async fn get_well_known() -> poem::Result<impl IntoResponse> {
    let well_known = WellKnownResponse {
        api: BIND_API.to_string(),
    };
    Ok(Json(well_known))
}
