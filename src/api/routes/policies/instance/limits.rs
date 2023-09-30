use crate::database::entities::Config;
use poem::{
    handler,
    web::{Data, Json},
    IntoResponse,
};

#[handler]
pub async fn limits(Data(cfg): Data<&Config>) -> impl IntoResponse {
    Json(serde_json::to_value(&cfg.limits).unwrap())
}
