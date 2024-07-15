use chorus::types::jwt::Claims;
use poem::{handler, IntoResponse, web::Data};
use sqlx::MySqlPool;

use crate::database::entities::Config;

#[handler]
pub async fn create_bot(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Data(config): Data<&Config>,
) -> poem::Result<impl IntoResponse> {
}
