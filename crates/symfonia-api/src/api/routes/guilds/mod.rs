// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{GuildCreateSchema, jwt::Claims};
use poem::{
	IntoResponse, Route, get, handler, patch, post, put,
	web::{Data, Json},
};
use sqlx::PgPool;
use util::{
	SharedEventPublisherMap,
	entities::{Config, Guild, User},
	errors::{Error, UserError},
};

mod id;
mod templates;

pub fn setup_routes() -> Route {
	Route::new()
		.at("/", post(create_guild))
		.at(
			"/templates/:code",
			get(templates::get_template).post(templates::create_guild_from_template),
		)
		.at(
			"/:guild_id",
			get(id::get_guild)
				.patch(id::modify_guild)
				.delete(id::delete_guild)
				.post(id::delete_guild),
		)
		.at(
			"/:guild_id/discovery-requirements",
			get(id::discovery_requirements::discovery_requirements),
		)
		.at(
			"/:guild_id/channels",
			get(id::channels::get_channels)
				.post(id::channels::create_channel)
				.patch(id::channels::reorder_channels_route),
		)
		.at("/:guild_id/invites", get(id::invites::get_invites))
		.at("/:guild_id/bans", get(id::bans::get_bans))
		.at("/:guild_id/bans/search", post(id::bans::search))
		.at("/:guild_id/bulk-ban", post(id::bans::bulk_ban))
		.at(
			"/:guild_id/bans/:user_id",
			put(id::bans::create_ban).get(id::bans::get_banned_user).delete(id::bans::delete_ban),
		)
		.at("/:guild_id/emojis", get(id::emoji::get_emojis).post(id::emoji::create_emoji))
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
		.at(
			"/:guild_id/vanity-url",
			get(id::vanity_url::get_vanity).patch(id::vanity_url::set_vanity),
		)
		.at(
			"/:guild_id/welcome-screen",
			get(id::welcome_screen::get_welcome_screen)
				.patch(id::welcome_screen::modify_welcome_screen),
		)
		.at("/:guild_id/members", get(id::members::get_members))
		.at("/:guild_id/members/search", post(id::members::search_members))
		.at(
			"/:guild_id/members/:member_id",
			get(id::members::id::get_member)
				.patch(id::members::id::modify_member)
				.put(id::members::id::join_guild)
				.delete(id::members::id::remove_member),
		)
		.at("/:guild_id/members/:member_id/nick", patch(id::members::id::nick::change_nickname))
		.at(
			"/:guild_id/members/:member_id/roles/:role_id",
			put(id::members::id::roles::add_role).delete(id::members::id::roles::remove_role),
		)
		.at(
			"/:guild_id/roles",
			get(id::roles::get_roles)
				.post(id::roles::create_role)
				.patch(id::roles::update_position),
		)
		.at("/:guild_id/roles/member-counts", get(id::roles::member_counts::count_by_members))
		.at(
			"/:guild_id/roles/:role_id",
			get(id::roles::id::get_role)
				.delete(id::roles::id::delete_role)
				.patch(id::roles::id::modify_role),
		)
		.at("/:guild_id/roles/:role_id/member-ids", get(id::roles::id::member_ids::get_member_ids))
		.at("/:guild_id/roles/:role_id/members", patch(id::roles::id::members::bulk_assign_roles))
		.at("/:guild_id/voice-states/:user_id", patch(id::voice_states::update_voice_state))
}

#[handler]
pub async fn create_guild(
	Data(db): Data<&PgPool>,
	Data(publisher_map): Data<&SharedEventPublisherMap>,
	Data(cfg): Data<&Config>,
	Data(claims): Data<&Claims>,
	Json(payload): Json<GuildCreateSchema>,
) -> poem::Result<impl IntoResponse> {
	let guild_name = if let Some(name) = payload.name {
		name
	} else {
		let user =
			User::get_by_id(db, claims.id).await?.ok_or(Error::User(UserError::InvalidUser))?;
		format!("{}'s Guild", user.username)
	};

	// TODO: Handle guild templates

	let guild = Guild::create(
		db,
		publisher_map.clone(),
		cfg,
		&guild_name,
		payload.icon,
		claims.id,
		&payload.channels.unwrap_or_default(),
	)
	.await?;

	Ok(Json(guild))
}
