use chorus::types::jwt::Claims;
use chorus::types::Snowflake;
use poem::{handler, IntoResponse};
use poem::web::{Data, Json, Path};
use sqlx::MySqlPool;

use crate::database::entities::Guild;
use crate::errors::{Error, GuildError};

#[handler]
pub async fn get_invites(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(guild_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    // TODO: Check if the user is allowed to see the invites of this guild

    let invites = guild.get_invites(db).await?;

    Ok(Json(invites))
}
