use chorus::types::{ApplicationModifySchema, jwt::Claims, Snowflake};
use poem::{
    handler,
    IntoResponse,
    web::{Data, Json, Path},
};
use sqlx::MySqlPool;

use crate::{
    database::entities::Application,
    errors::{ApplicationError, Error},
};

mod bot;

#[handler]
pub async fn get_application(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(application_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
    let application = Application::get_by_id(db, application_id)
        .await?
        .ok_or(Error::Application(ApplicationError::NotFound))?;

    if application.owner_id != claims.id {
        return Err(Error::Application(ApplicationError::UnauthorizedAction).into());
    }

    Ok(Json(application.into_inner()))
}

#[handler]
pub async fn update_application(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(application_id): Path<Snowflake>,
    Json(payload): Json<ApplicationModifySchema>,
) -> poem::Result<impl IntoResponse> {
    let mut application = Application::get_by_id(db, application_id)
        .await?
        .ok_or(Error::Application(ApplicationError::NotFound))?;

    if application.owner_id != claims.id {
        return Err(Error::Application(ApplicationError::UnauthorizedAction).into());
    }

    // TODO: Handle OTP verification

    application.populate_relations(db).await?;

    if let Some(name) = payload.name {
        application.name = name;
    }
    if let Some(bot) = application.bot.as_mut() {
        bot.bio = payload.description.clone();
    }

    application.description = payload.description;

    if let Some(icon) = payload.icon {
        application.icon = Some(icon);

        // TODO: Handle CDN
    }
    if let Some(interactions_endpoint_url) = payload.interactions_endpoint_url {
        application.interactions_endpoint_url = Some(interactions_endpoint_url);
    }
    if let Some(max_participants) = payload.max_participants {
        // application.max_participants = max_participants;
    }
    if let Some(privacy_policy_url) = payload.privacy_policy_url {
        application.privacy_policy_url = Some(privacy_policy_url);
    }
    if let Some(role_connections_verification_url) = payload.role_connections_verification_url {
        // application.role_connections_verification_url = Some(role_connections_verification_url);
    }
    if let Some(tags) = payload.tags {
        application.tags = Some(sqlx::types::Json(tags));
    }
    if let Some(terms_of_service_url) = payload.terms_of_service_url {
        application.terms_of_service_url = Some(terms_of_service_url);
    }
    if let Some(bot_public) = payload.bot_public {
        application.bot_public = bot_public;
    }
    if let Some(grant_required) = payload.bot_require_code_grant {
        application.bot_require_code_grant = grant_required;
    }
    if let Some(flags) = payload.flags {
        application.flags = flags;
    }
    application.save(db).await?;

    Ok(Json(application.into_inner()))
}

#[handler]
pub async fn delete_application(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(application_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
    let application = Application::get_by_id(db, application_id)
        .await?
        .ok_or(Error::Application(ApplicationError::NotFound))?;

    if application.owner_id != claims.id {
        return Err(Error::Application(ApplicationError::UnauthorizedAction).into());
    }

    application.delete(db).await?;

    Ok(Json(serde_json::Value::Null))
}
