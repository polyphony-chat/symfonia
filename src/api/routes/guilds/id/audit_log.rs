use chorus::types::{AuditLogObject, GetAuditLogsQuery, PermissionFlags, Snowflake};
use poem::{
    handler,
    IntoResponse,
    web::{Data, Json, Path, Query},
};
use sqlx::MySqlPool;

use crate::{
    database::entities::{AuditLogEntry, Guild, User},
    errors::{Error, GuildError},
};

#[handler]
pub async fn get_audit_logs(
    Data(db): Data<&MySqlPool>,
    Data(authed_user): Data<&User>,
    Path(guild_id): Path<Snowflake>,
    Query(query): Query<GetAuditLogsQuery>,
) -> poem::Result<impl IntoResponse> {
    let guild = Guild::get_by_id(db, guild_id)
        .await?
        .ok_or(Error::Guild(GuildError::InvalidGuild))?;

    let our_member = guild
        .get_member(db, authed_user.id)
        .await?
        .ok_or(Error::Guild(GuildError::MemberNotFound))?;

    if !our_member
        .permissions
        .has_permission(PermissionFlags::VIEW_AUDIT_LOG)
    {
        return Err(Error::Guild(GuildError::InsufficientPermissions).into());
    }

    let logs = AuditLogEntry::get_by_guild(
        db,
        guild.id,
        query.before,
        query.after,
        query.limit.unwrap_or(50),
        query.user_id,
        query.action_type,
    )
    .await?;

    let mut audit_log = AuditLogObject {
        audit_log_entries: logs.into_iter().map(|e| e.into_inner()).collect(),
        application_commands: vec![],
        auto_moderation_rules: vec![],
        guild_scheduled_events: vec![],
        integrations: vec![],
        threads: vec![],
        users: vec![],
        webhooks: vec![],
    };

    Ok(Json(audit_log))
}
