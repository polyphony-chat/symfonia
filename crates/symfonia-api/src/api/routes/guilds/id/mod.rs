// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{
	ChannelType, GuildModifySchema, PermissionFlags, PermissionOverwrite, PermissionOverwriteType,
	Rights, Snowflake,
	jwt::Claims,
	types::guild_configuration::{GuildFeatures, GuildFeaturesList},
};
use itertools::Itertools;
use poem::{
	IntoResponse, Response, handler,
	http::StatusCode,
	web::{Data, Json, Path},
};
use sqlx::PgPool;
use util::{
	entities::{Channel, Guild, GuildMember, Role, User},
	errors::{ChannelError, Error, GuildError},
};

mod audit_log;
pub(crate) mod bans;
pub mod channels;
pub(crate) mod discovery_requirements;
pub(crate) mod emoji;
pub(crate) mod invites;
pub(crate) mod members;
mod messages;
pub(crate) mod prune;
pub(crate) mod roles;
pub(crate) mod stickers;
pub(crate) mod vanity_url;
pub(crate) mod voice_states;
pub(crate) mod welcome_screen;

#[handler]
pub async fn get_guild(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(guild_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
	let mut guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	let channels = Channel::get_by_guild_id(db, guild_id).await?;

	guild.channels = channels.into_iter().map(|c| c.into_inner()).collect();

	let roles = Role::get_by_guild(db, guild_id).await?;

	guild.roles = roles.into_iter().map(|r| r.into_inner()).collect();

	let member = GuildMember::get_by_id(db, claims.id, guild_id)
		.await?
		.ok_or(Error::Guild(GuildError::MemberNotFound))?;

	guild.joined_at = Some(member.joined_at);
	guild.owner = guild.owner_id.map(|id| id == claims.id);

	Ok(Json(guild.into_inner()))
}

#[handler]
pub async fn modify_guild(
	Data(db): Data<&PgPool>,
	Data(authed_user): Data<&User>,
	Path(guild_id): Path<Snowflake>,
	Json(payload): Json<GuildModifySchema>,
) -> poem::Result<impl IntoResponse> {
	let mut guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	let member = guild
		.get_member(db, authed_user.id)
		.await?
		.ok_or(Error::Guild(GuildError::MemberNotFound))?;

	if authed_user.rights.has(Rights::MANAGE_GUILDS, true)
		&& !member.permissions.has_permission(PermissionFlags::MANAGE_GUILD)
	{
		return Err(Error::Guild(
			GuildError::InsufficientPermissions, /*(
													 PermissionFlags::MANAGE_GUILD,
												 )*/
		)
		.into());
	}

	// TODO: Handle guild icon/banner/splash on CDN

	if let Some(features) = payload.features {
		let diff = guild
			.features
			.iter()
			.filter(|f| !features.contains(f))
			.chain(&features)
			.unique()
			.cloned()
			.collect::<Vec<_>>();

		const MUTABLE_FEATURES: &[GuildFeatures] = &[
			GuildFeatures::Community,
			GuildFeatures::InvitesDisabled,
			GuildFeatures::Discoverable,
		];

		if diff.iter().any(|f| !MUTABLE_FEATURES.contains(f)) {
			return Err(Error::Guild(GuildError::FeatureIsImmutable).into());
		}
		guild.features = GuildFeaturesList::from(diff);
	}

	if let Some(pub_update_channel_id) = payload.public_updates_channel_id {
		if pub_update_channel_id == Snowflake(1) {
			let channel = Channel::create(
				db,
				ChannelType::GuildText,
				Some(String::from("moderator-only")),
				false,
				Some(guild.id),
				None,
				false,
				false,
				true,
				false,
				vec![PermissionOverwrite {
					id: guild.id, // @everyone
					overwrite_type: PermissionOverwriteType::Role,
					allow: PermissionFlags::empty(),
					deny: PermissionFlags::VIEW_CHANNEL,
				}],
			)
			.await?;

			// TODO: Update guild channel ordering (position)

			guild.public_updates_channel_id = Some(channel.id);
		} else {
			Channel::get_by_id(db, pub_update_channel_id)
				.await?
				.ok_or(Error::Channel(ChannelError::InvalidChannel))?;
			guild.public_updates_channel_id = Some(pub_update_channel_id);
		}
	}

	if let Some(rules_channel_id) = payload.rules_channel_id {
		if rules_channel_id == Snowflake(1) {
			let channel = Channel::create(
				db,
				ChannelType::GuildText,
				Some(String::from("rules")),
				false,
				Some(guild.id),
				None,
				false,
				false,
				true,
				false,
				vec![PermissionOverwrite {
					id: guild.id, // @everyone
					overwrite_type: PermissionOverwriteType::Role,
					allow: PermissionFlags::empty(),
					deny: PermissionFlags::SEND_MESSAGES,
				}],
			)
			.await?;
		} else {
			Channel::get_by_id(db, rules_channel_id)
				.await?
				.ok_or(Error::Channel(ChannelError::InvalidChannel))?;
			guild.rules_channel_id = Some(rules_channel_id);
		}
	}

	guild.save(db).await?;

	// TODO: Emit event 'GUILD_UPDATE'

	Ok(Json(guild.into_inner()))
}

#[handler]
pub async fn delete_guild(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(guild_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	// TODO: Check if the user is the owner of the guild

	guild.delete(db).await?;

	// TODO: Emit event 'GUILD_DELETE'

	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}
