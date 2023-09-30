mod domain;
mod limits;

use crate::database::entities::Config;
pub use domain::*;
pub use limits::*;
use poem::{
    get, handler,
    web::{Data, Json},
    IntoResponse, Route,
};

#[handler]
pub async fn general_config(Data(cfg): Data<&Config>) -> impl IntoResponse {
    Json(serde_json::to_value(&cfg.general).unwrap())
}

pub fn setup_routes() -> Route {
    Route::new()
        .at("/", get(general_config))
        .at("/limits", get(limits))
        .at("/domains", get(domain))
}
