use chorus::types::GuildCreateSchema;
use chorus::types::jwt::Claims;
use poem::{get, handler, IntoResponse, post, Route};
use poem::web::{Data, Json};
use sqlx::MySqlPool;

use crate::api::routes::guilds::id::channels::{create_channel, get_channels};
use crate::api::routes::guilds::id::get_guild;
use crate::database::entities::{Config, Guild, User};
use crate::errors::{Error, UserError};

mod id;

pub fn setup_routes() -> Route {
    Route::new()
        .at("/", post(create_guild))
        .at("/:guild_id", get(get_guild))
        .at(
            "/:guild_id/channels",
            get(get_channels).post(create_channel),
        )
        .at("/:guild_id/invites", get(id::invites::get_invites))
}

#[handler]
pub async fn create_guild(
    Data(db): Data<&MySqlPool>,
    Data(cfg): Data<&Config>,
    Data(claims): Data<&Claims>,
    Json(payload): Json<GuildCreateSchema>,
) -> poem::Result<impl IntoResponse> {
    let guild_name = if let Some(name) = payload.name {
        name
    } else {
        let user = User::get_by_id(db, claims.id)
            .await?
            .ok_or(Error::User(UserError::InvalidUser))?;
        format!("{}'s Guild", user.username)
    };

    // TODO: Handle guild templates

    let guild = Guild::create(
        db,
        cfg,
        &guild_name,
        payload.icon,
        claims.id,
        vec![], // TODO: payload.channels.unwrap_or_default(),
    )
    .await?;

    Ok(Json(guild))
}
