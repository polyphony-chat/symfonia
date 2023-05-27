use crate::database::entities::Config;
use chorus::types::APIError;
use poem::{
    handler,
    web::{Data, Json},
    IntoResponse,
};
use serde_json::json;

#[handler]
pub async fn stats(
    Data(db): Data<&sqlx::MySqlPool>,
    Data(cfg): Data<&Config>,
) -> Result<impl IntoResponse, APIError> {
    if !cfg.security.stats_world_readable {
        // TODO: Check requester rights
    }

    Ok(Json(json!({
        "counts": {
            "user": 0,
            "guild": 0,
            "message": 0,
            "members": 0
        }
    })))
}
