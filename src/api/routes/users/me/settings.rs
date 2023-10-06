use crate::database::entities::User;
use crate::errors::{Error, UserError};
use chorus::types::jwt::Claims;
use chorus::types::UserSettings;
use poem::web::{Data, Json};
use poem::{handler, IntoResponse};
use sqlx::MySqlPool;

#[handler]
pub async fn get_settings(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
) -> poem::Result<impl IntoResponse> {
    let user = User::get_by_id(db, claims.id)
        .await?
        .ok_or(Error::User(UserError::InvalidUser))?;

    Ok(Json(user.settings))
}

#[handler]
pub async fn update_settings(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Json(settings): Json<UserSettings>,
) -> poem::Result<impl IntoResponse> {
    let mut user = User::get_by_id(db, claims.id)
        .await?
        .ok_or(Error::User(UserError::InvalidUser))?;

    user.settings =
        crate::database::entities::UserSettings::consume(settings, user.settings_index as u64);
    // TODO: user.settings.update(db).await.map_err(Error::SQLX)?;

    Ok(Json(user.settings))
}
