use crate::database::entities::User;
use crate::errors::{Error, UserError};
use chorus::types::jwt::Claims;
use poem::web::{Data, Json};
use poem::{get, handler, IntoResponse, Route};
use sqlx::MySqlPool;

pub fn setup_routes() -> Route {
    Route::new().at("/", get(get_data))
}

#[handler]
pub async fn get_data(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
) -> poem::Result<impl IntoResponse> {
    let user = User::get_by_id(db, claims.id)
        .await
        .unwrap()
        .ok_or(Error::User(UserError::InvalidUser))
        .unwrap();

    Ok(Json(user))
}
