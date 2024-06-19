use chorus::types::{GuildCreateSchema, jwt::Claims};
use poem::{
    get, handler, IntoResponse, post,
    put,
    Route, web::{Data, Json},
};
use sqlx::MySqlPool;

use crate::{
    api::routes::guilds::id::{
        channels::{create_channel, get_channels},
        get_guild,
    },
    database::entities::{Config, Guild, User},
    errors::{Error, UserError},
};

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
        .at("/:guild_id/bans", get(id::bans::get_bans))
        .at("/:guild_id/bans/search", post(id::bans::search))
        .at("/:guild_id/bulk-ban", post(id::bans::bulk_ban))
        .at(
            "/:guild_id/bans/:user_id",
            put(id::bans::create_ban)
                .get(id::bans::get_banned_user)
                .delete(id::bans::delete_ban),
        )
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
