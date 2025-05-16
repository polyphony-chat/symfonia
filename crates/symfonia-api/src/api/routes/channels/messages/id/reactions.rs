// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{PartialEmoji, Reaction, ReactionQuerySchema, Snowflake, jwt::Claims};
use poem::{
	IntoResponse, Response, handler,
	web::{Data, Json, Path, Query},
};
use reqwest::StatusCode;
use sqlx::PgPool;
use util::{
	entities::{Channel, Emoji, GuildMember, Message, User},
	errors::{ChannelError, Error, GuildError, ReactionError, UserError},
};

#[handler]
pub async fn add_reaction(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path((channel_id, message_id)): Path<(Snowflake, Snowflake)>,
	Path((emoji, user_id)): Path<(String, String)>,
) -> poem::Result<impl IntoResponse> {
	if user_id != "@me" {
		return Err(Error::User(UserError::InvalidUser).into());
	}

	let mut partial_emoji =
		get_partial_emoji(&emoji).ok_or(Error::Reaction(ReactionError::Invalid))?;

	let channel = Channel::get_by_id(db, channel_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;
	let mut message = Message::get_by_id(db, channel_id, message_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidMessage))?;

	if let Some(emoji_id) = partial_emoji.id {
		if let Some(external_emoji) = Emoji::get_by_id(db, emoji_id).await? {
			if message.get_reaction(&partial_emoji).is_none()
				&& channel.guild_id.map(|id| external_emoji.guild_id.eq(&id)).unwrap_or_default()
			{
				// TODO: check permissions 'USE_EXTERNAL_EMOJIS'
			}

			if let Some(name) = &external_emoji.name {
				partial_emoji.name = name.to_owned();
			}
			partial_emoji.animated = external_emoji.animated.unwrap_or_default();
		}
	}

	// TODO: Check permissions 'ADD_REACTIONS'

	if let Some(reaction) = message.get_reaction_mut(&partial_emoji) {
		if reaction.user_ids.contains(&claims.id) {
			// TODO: No error thrown for compatibility with discord, may change in the
			// future
			return Ok(Response::builder().status(StatusCode::NO_CONTENT).finish());
		}

		reaction.count += 1.into();
		reaction.user_ids.push(claims.id);
	} else {
		let new_reaction = Reaction {
			emoji: partial_emoji.clone(),
			count: 1.into(),
			burst_count: 0.into(),
			me: true,
			burst_me: false,
			user_ids: vec![claims.id],
			burst_colors: vec![],
		};
		if let Some(reactions) = message.reactions.as_mut() {
			reactions.push(new_reaction);
		} else {
			message.reactions = Some(sqlx::types::Json(vec![new_reaction]));
		}
	}

	message.save(db).await?;

	if let Some(guild_id) = channel.guild_id {
		let _member = GuildMember::get_by_id(db, claims.id, guild_id)
			.await?
			.ok_or(Error::Guild(GuildError::InvalidGuild))?;

		// TODO: emit events 'MESSAGE_REACTION_ADD'
	}

	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}

#[handler]
pub async fn delete_all_reactions(
	Data(db): Data<&PgPool>,
	Path((channel_id, message_id)): Path<(Snowflake, Snowflake)>,
) -> poem::Result<impl IntoResponse> {
	// TODO: Check permissions
	let mut message = Message::get_by_id(db, channel_id, message_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidMessage))?;

	message.clear_reactions(db).await?;

	// TODO: Emit event

	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}

#[handler]
pub async fn delete_reaction(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path((channel_id, message_id)): Path<(Snowflake, Snowflake)>,
	Path((emoji, user_id)): Path<(String, String)>,
) -> poem::Result<impl IntoResponse> {
	let partial_emoji = get_partial_emoji(&emoji).ok_or(Error::Reaction(ReactionError::Invalid))?;

	let channel = Channel::get_by_id(db, channel_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidChannel))?;

	let mut message = Message::get_by_id(db, channel.id, message_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidMessage))?;

	let mut uid;
	if user_id.eq("@me") {
		uid = claims.id;
	} else {
		uid = Snowflake(user_id.parse::<u64>().map_err(|_| Error::User(UserError::InvalidUser))?);

		// TODO: Check permissions 'MANAGE_MESSAGES'
	}

	message.remove_reaction(db, partial_emoji).await?;

	// TODO: Emit event

	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}

#[handler]
pub async fn get_reaction(
	Data(db): Data<&PgPool>,
	Path((channel_id, message_id)): Path<(Snowflake, Snowflake)>,
	Path(emoji): Path<String>,
	Query(query): Query<ReactionQuerySchema>,
) -> poem::Result<impl IntoResponse> {
	let emoji = get_partial_emoji(&emoji).ok_or(Error::Reaction(ReactionError::Invalid))?;

	let message = Message::get_by_id(db, channel_id, message_id)
		.await?
		.ok_or(Error::Channel(ChannelError::InvalidMessage))?;

	let Some(reaction) = message.reactions.as_ref().and_then(|reactions| {
		reactions.iter().find(|r| {
			if emoji.id.is_some() && r.emoji.id.is_some() {
				emoji.id == r.emoji.id
			} else {
				emoji.name.eq(&r.emoji.name)
			}
		})
	}) else {
		return Err(Error::Reaction(ReactionError::NotFound).into());
	};

	let mut limit = query.limit.unwrap_or(25).min(100);

	let users = User::get_by_id_list(db, &reaction.user_ids, query.after, limit.into()).await?;

	let public_projections = users.iter().map(|u| u.to_public_user()).collect::<Vec<_>>();

	Ok(Json(public_projections))
}

pub fn get_partial_emoji(emoji: &str) -> Option<PartialEmoji> {
	let clean_emoji = percent_encoding::percent_decode_str(emoji).decode_utf8().ok()?;
	if let Some((name, snowflake)) = emoji.split_once(':') {
		let name = name.to_owned();
		let snowflake = Snowflake(snowflake.parse::<u64>().ok()?);
		Some(PartialEmoji { name, id: Some(snowflake), animated: false })
	} else {
		Some(PartialEmoji { name: clean_emoji.to_string(), id: None, animated: false })
	}
}
