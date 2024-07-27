/* 
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;

use chorus::types::{jwt::Claims, Snowflake};
use poem::{
    handler,
    IntoResponse,
    web::{Data, Json, Path},
};
use sqlx::MySqlPool;

use crate::{
    database::entities::{Guild, GuildMember},
    errors::{Error, GuildError},
};

#[handler]
pub async fn count_by_members(
    Data(db): Data<&MySqlPool>,
    Data(claims): Data<&Claims>,
    Path(guild_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    if !guild.has_member(db, claims.id).await? {
        return Err(Error::Guild(GuildError::MemberNotFound).into());
    }

    let role_ids = guild
        .get_roles(db)
        .await?
        .into_iter()
        .map(|r| r.id)
        .collect::<Vec<_>>();

    let mut counts = HashMap::new();

    // TODO: optimize this into one query
    for role_id in role_ids {
        let count = GuildMember::count_by_role(db, role_id).await?;
        counts.insert(role_id, count);
    }

    Ok(Json(counts))
}
