use chorus::types::{GuildCreateSchema, jwt::Claims};
use poem::{
    get, handler, IntoResponse, post,
    put,
    Route, web::{Data, Json},
};
use sqlx::MySqlPool;

use crate::{
    database::entities::{Config, Guild, User},
    errors::{Error, UserError},
};

mod id;

pub fn setup_routes() -> Route {
    Route::new()
        .at("/", post(create_guild))
        .at(
            "/:guild_id",
            get(id::get_guild)
                .patch(id::modify_guild)
                .delete(id::delete_guild),
        )
        .at(
            "/:guild_id/discovery-requirements",
            get(id::discovery_requirements::discovery_requirements),
        )
        .at(
            "/:guild_id/channels",
            get(id::channels::get_channels)
                .post(id::channels::create_channel)
                .patch(id::channels::reoder_channels),
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
        .at(
            "/:guild_id/emojis",
            get(id::emoji::get_emojis).post(id::emoji::create_emoji),
        )
        .at(
            "/:guild_id/emojis/:emoji_id",
            get(id::emoji::get_emoji)
                .patch(id::emoji::modify_emoji)
                .delete(id::emoji::delete_emoji),
        )
        .at(
            "/:guild_id/prune",
            get(id::prune::prune_members_dry_run).post(id::prune::prune_members),
        )
        .at(
            "/:guild_id/stickers",
            get(id::stickers::get_stickers).post(id::stickers::create_sticker),
        )
        .at(
            "/:guild_id/stickers/:sticker_id",
            get(id::stickers::get_sticker)
                .patch(id::stickers::modify_sticker)
                .delete(id::stickers::delete),
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
