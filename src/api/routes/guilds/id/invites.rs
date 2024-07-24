use chorus::types::{jwt::Claims, GetInvitesSchema, Snowflake};
use poem::{
    handler,
    web::{Data, Json, Path, Query},
    IntoResponse,
};
use sqlx::MySqlPool;

use crate::{
    database::entities::Guild,
    errors::{Error, GuildError},
};

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

        for invite in invites.iter_mut() {
            invite.populate_relations(db).await?;
        }
    }

    Ok(Json(invites))
}
