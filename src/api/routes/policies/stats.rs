use chorus::types::APIError;
use poem::{
    handler,
    IntoResponse,
    web::{Data, Json},
};
use serde_json::json;

use crate::database::entities::{Config, Guild, GuildMember, Message, User};

#[handler]
pub async fn stats(
    Data(db): Data<&sqlx::MySqlPool>,
    Data(cfg): Data<&Config>,
) -> poem::Result<impl IntoResponse> {
    if !cfg.security.stats_world_readable {
        // TODO: Check requester rights
    }

    let users = User::count(db).await?;
    let guilds = Guild::count(db).await?;
    let messages = Message::count(db).await?;
    let members = GuildMember::count(db).await?;

    Ok(Json(json!({
        "counts": {
            "user": users,
            "guild": guilds,
            "message": messages,
            "members": members
        }
    })))
}
