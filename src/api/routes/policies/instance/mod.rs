mod domain;
mod limits;

use crate::database::entities::Config;
pub use domain::*;
pub use limits::*;
use poem::{
    handler,
    web::{Data, Json},
    IntoResponse,
};

#[handler]
pub async fn general_config(Data(cfg): Data<&Config>) -> impl IntoResponse {
    Json(serde_json::to_value(&cfg.general).unwrap())
}
