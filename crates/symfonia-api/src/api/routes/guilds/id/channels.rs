// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::{
	ChannelModifySchema, ChannelType, ModifyChannelPositionsSchema, Snowflake, jwt::Claims,
};
use poem::{
	IntoResponse, Response, handler,
	web::{Data, Json, Path},
};
use reqwest::StatusCode;
use sqlx::PgPool;
use util::{
	entities::{Channel, Guild},
	errors::{Error, GuildError},
};

#[handler]
pub async fn get_channels(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(guild_id): Path<Snowflake>,
) -> poem::Result<impl IntoResponse> {
	let channels = Channel::get_by_guild_id(db, guild_id).await?;

	Ok(Json(channels.into_iter().map(|c| c.into_inner()).collect::<Vec<_>>()))
}

#[handler]
pub async fn create_channel(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(guild_id): Path<Snowflake>,
	Json(payload): Json<ChannelModifySchema>,
) -> poem::Result<impl IntoResponse> {
	let channel = Channel::create(
		db,
		payload.channel_type.unwrap_or(ChannelType::GuildText),
		payload.name,
		payload.nsfw.unwrap_or_default(),
		Some(guild_id),
		payload.parent_id,
		false,
		false,
		false,
		false,
		payload.permission_overwrites.unwrap_or_else(std::vec::Vec::new),
	)
	.await?;

	Ok(Json(channel.into_inner()).with_status(StatusCode::CREATED))
}

#[handler]
pub async fn reorder_channels_route(
	Data(db): Data<&PgPool>,
	Data(claims): Data<&Claims>,
	Path(guild_id): Path<Snowflake>,
	Json(payload): Json<ModifyChannelPositionsSchema>,
) -> poem::Result<impl IntoResponse> {
	let guild =
		Guild::get_by_id(db, guild_id).await?.ok_or(Error::Guild(GuildError::InvalidGuild))?;

	let mut channels = Channel::get_by_guild_id(db, guild_id).await?;

	if let Some(pos) = payload.position {
		let mut channels = channels.iter().filter(|c| c.position.is_some()).cloned().collect();
		//        Channel::reorder(db, guild.id, payload.id, pos).await?;
		reorder_channels(payload.id, pos as i32, &mut channels);

		for channel in channels {
			channel.save(db).await?;
			// TODO: emit events
		}
	}

	// TODO: properly handle parents

	Ok(Response::builder().status(StatusCode::NO_CONTENT).finish())
}

pub fn reorder_channels(target_id: Snowflake, new_position: i32, channels: &mut Vec<Channel>) {
	channels.sort_by(|a, b| a.position.cmp(&b.position));

	if let Some(channel_idx) = channels.iter().position(|c| c.id == target_id) {
		let mut target_channel = channels.remove(channel_idx);
		let old_position = target_channel.position;
		target_channel.position = Some(new_position);

		if target_channel.position > old_position {
			for channel in channels.iter_mut() {
				if channel.position > old_position && channel.position <= Some(new_position) {
					if let Some(pos) = channel.position.as_mut() {
						*pos -= 1;
					}
				}
			}
		} else {
			for channel in channels.iter_mut() {
				if channel.position < old_position && channel.position >= Some(new_position) {
					if let Some(pos) = channel.position.as_mut() {
						*pos += 1;
					}
				}
			}
		}

		let insert_pos = channels
			.binary_search_by(|c| c.position.cmp(&target_channel.position))
			.unwrap_or_else(|e| e);

		channels.insert(insert_pos, target_channel);
	}
}

#[cfg(test)]
mod tests {
	use chorus::types::Snowflake;
	use util::entities::Channel;

	#[test]
	fn test_reorder_channels() {
		let target_id = Snowflake::default();

		let mut channels = vec![
			Channel {
				inner: chorus::types::Channel {
					id: target_id,
					position: Some(0),
					..Default::default()
				},
				..Default::default()
			},
			Channel {
				inner: chorus::types::Channel {
					id: Snowflake::default(),
					position: Some(1),
					..Default::default()
				},
				..Default::default()
			},
			Channel {
				inner: chorus::types::Channel {
					id: Snowflake::default(),
					position: Some(2),
					..Default::default()
				},
				..Default::default()
			},
			Channel {
				inner: chorus::types::Channel {
					id: Snowflake::default(),
					position: Some(3),
					..Default::default()
				},
				..Default::default()
			},
			Channel {
				inner: chorus::types::Channel {
					id: Snowflake::default(),
					position: Some(4),
					..Default::default()
				},
				..Default::default()
			},
			Channel {
				inner: chorus::types::Channel {
					id: Snowflake::default(),
					position: Some(5),
					..Default::default()
				},
				..Default::default()
			},
		];

		crate::api::routes::guilds::id::channels::reorder_channels(target_id, 3, &mut channels);
		println!("{}", serde_json::to_string_pretty(&channels).unwrap());

		assert_eq!(channels.iter().position(|c| c.id == target_id).unwrap_or(0), 3);
	}
}
