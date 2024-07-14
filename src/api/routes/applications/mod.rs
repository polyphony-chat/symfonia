use chorus::types::{ApplicationCreateSchema, ApplicationFlags, Rights};
use poem::{
    get, handler,
    IntoResponse,
    Route, web::{Data, Json, Query},
};
use sqlx::MySqlPool;

use crate::database::entities::{Application, Config, User};

pub fn setup_routes() -> Route {
    Route::new().at("/", get(get_applications))
}

#[handler]
pub async fn get_applications(
    Data(db): Data<&MySqlPool>,
    Data(authed_user): Data<&User>,
    Query(with_team_applications): Query<bool>,
) -> poem::Result<impl IntoResponse> {
    let mut applications = Application::get_by_owner(db, authed_user.id).await?;

    if with_team_applications {
        let team_members = authed_user.get_team_member(db).await?;
        for member in &team_members {
            let team_applications = Application::get_by_team(db, member.team_id).await?;
            applications.extend(team_applications);
        }
    }

    for application in &mut applications {
        application.populate_relations(db).await?;
    }

    Ok(Json(applications))
}

#[handler]
pub async fn create_application(
    Data(db): Data<&MySqlPool>,
    Data(authed_user): Data<&User>,
    Data(config): Data<&Config>,
    Json(payload): Json<ApplicationCreateSchema>,
) -> poem::Result<impl IntoResponse> {
    if authed_user.rights.has(Rights::CREATE_APPLICATIONS, true) {
        todo!("Make an error type for this")
    }

    // TODO: Handle icon and cover image

    let application = Application::create(
        db,
        config,
        &payload.name,
        &payload.description.unwrap_or_default(),
        authed_user.id,
        "IMPLEMENT ME",
        payload.flags.unwrap_or(ApplicationFlags::empty()),
        true,
        payload.redirect_uris,
    )
    .await?;

    Ok(Json(application))
}
