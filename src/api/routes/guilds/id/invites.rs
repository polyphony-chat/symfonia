use chorus::types::{GetInvitesSchema, Snowflake};
use chorus::types::jwt::Claims;
use poem::{handler, IntoResponse};
use poem::web::{Data, Json, Path, Query};
use sqlx::MySqlPool;

use crate::database::entities::Guild;
use crate::errors::{Error, GuildError};

#[handler]
pub async fn get_invites(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(guild_id): Path<Snowflake>,
    Query(query): Query<GetInvitesSchema>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    // TODO: Check if the user is allowed to see the invites of this guild

    let mut invites = guild.get_invites(db).await?;

    if query.with_counts.unwrap_or_default() {
        // TODO: Get approximate member count
        // TODO: Get approximate online member count (presences)

        invites.iter_mut().for_each(|invite| {});
    }

    Ok(Json(invites))
}
