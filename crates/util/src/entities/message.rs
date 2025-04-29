// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

use chorus::types::{
	ChannelMessagesAnchor, MessageFlags, MessageModifySchema, MessageSearchQuery,
	MessageSendSchema, MessageType, PartialEmoji, Reaction, Snowflake,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, QueryBuilder, Row};
use sqlx_pg_uint::PgU64;

use crate::{
	entities::User,
	errors::{ChannelError, Error, ReactionError},
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
	#[sqlx(flatten)]
	#[serde(flatten)]
	inner: chorus::types::Message,
	pub author_id: Snowflake,
	pub guild_id: Option<Snowflake>,
	pub message_reference_id: Option<Snowflake>,
}

impl Deref for Message {
	type Target = chorus::types::Message;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for Message {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl Message {
	pub async fn create(
		db: &PgPool,
		payload: MessageSendSchema,
		guild_id: Option<Snowflake>,
		channel_id: Snowflake,
		author_id: Snowflake,
	) -> Result<Self, Error> {
		let mut flags = MessageFlags::empty();
		let mut message_reference_id = None;
		let mut referenced_message = None;
		if let Some(referenced) = &payload.message_reference {
			let message = Message::get_by_id(db, referenced.channel_id, referenced.message_id)
				.await?
				.ok_or(Error::Channel(ChannelError::InvalidMessage))?;
			flags.insert(MessageFlags::CROSSPOSTED | MessageFlags::IS_CROSSPOST);
			message_reference_id = Some(referenced.message_id);
			referenced_message = Some(Box::new(message.inner));
		}
		// TODO: Calculate other flags
		// TODO: Calculate mentions
		let mention_everyone = false;

		let ts = Utc::now();
		let new_message_id = Snowflake::generate();
		sqlx::query("INSERT INTO `messages` (`id`, `channel_id`, `guild_id`, `author_id`, `content`, `timestamp`, `tts`, `mention_everyone`, `embeds`, `attachments`, `reactions`, `nonce`, `type`, `activity`, `flags`, `message_reference`, `interaction`, `components`, `message_reference_id`, `message_type`) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, ?, ?, NULL, ?, ?, NULL, ?, ?, ?)")
            .bind(new_message_id)
            .bind(channel_id)
            .bind(guild_id)
            .bind(author_id)
            .bind(&payload.content)
            .bind(ts)
            .bind(payload.tts)
            .bind(mention_everyone)
            .bind(sqlx::types::Json(&payload.embeds))
            .bind(sqlx::types::Json(&payload.attachments))
            .bind(&payload.nonce)
            .bind(payload.message_type.unwrap_or(MessageType::Default))
            .bind(flags)
            .bind(sqlx::types::Json(&payload.message_reference))
            .bind(sqlx::types::Json(&payload.components))
            .bind(message_reference_id)
            .execute(db)
            .await?;

		Ok(Self {
			inner: chorus::types::Message {
				id: new_message_id,
				channel_id,
				author: None,
				content: payload.content,
				timestamp: ts,
				edited_timestamp: None,
				tts: payload.tts,
				mention_everyone,
				mentions: None,
				mention_roles: None,
				mention_channels: None,
				attachments: None, // TODO: payload.attachments,
				embeds: Default::default(),
				reactions: None,
				nonce: payload.nonce.map(serde_json::Value::String),
				pinned: false,
				webhook_id: None,
				message_type: payload.message_type.unwrap_or_default(),
				activity: None,
				application: None,
				application_id: None,
				message_reference: payload.message_reference.map(sqlx::types::Json),
				flags: Some(flags),
				referenced_message,
				interaction: None,
				thread: None,
				components: payload.components.map(sqlx::types::Json),
				sticker_items: None,
				stickers: None,
				role_subscription_data: None,
			},
			author_id,
			guild_id,
			message_reference_id,
		})
	}

	pub async fn get_by_nonce(
		db: &PgPool,
		channel_id: Snowflake,
		author_id: Snowflake,
		nonce: &str,
	) -> Result<Option<Self>, Error> {
		sqlx::query_as(
			"SELECT * FROM `messages` WHERE `channel_id` = ? AND `author_id` = ? AND `nonce` = ?",
		)
		.bind(channel_id)
		.bind(author_id)
		.bind(nonce)
		.fetch_optional(db)
		.await
		.map_err(Error::Sqlx)
	}

	pub async fn get_by_id(
		db: &PgPool,
		channel_id: Snowflake,
		id: Snowflake,
	) -> Result<Option<Self>, Error> {
		sqlx::query_as("SELECT * FROM `messages` WHERE `id` = ? AND `channel_id` = ?")
			.bind(id)
			.bind(channel_id)
			.fetch_optional(db)
			.await
			.map_err(Error::Sqlx)
	}

	pub async fn get_by_channel_id(
		db: &PgPool,
		channel_id: Snowflake,
		anchor: ChannelMessagesAnchor,
		limit: i32,
	) -> Result<Vec<Self>, Error> {
		match anchor {
            ChannelMessagesAnchor::Before(before_id) => {
                sqlx::query_as("SELECT * FROM `messages` WHERE `channel_id` = ? AND `id` < ? ORDER BY `timestamp` DESC LIMIT ?")
                    .bind(channel_id)
                    .bind(before_id)
                    .bind(limit)
                    .fetch_all(db)
                    .await
                    .map_err(Error::Sqlx)
            }
            ChannelMessagesAnchor::Around(around_id) => {
                let limit = limit / 2;
                if limit > 0 {
                    let mut upper: Vec<Message> = sqlx::query_as("SELECT * FROM `messages` WHERE `channel_id` = ? AND `id` > ? ORDER BY `timestamp` DESC LIMIT ?")
                        .bind(channel_id)
                        .bind(around_id)
                        .bind(limit)
                        .fetch_all(db)
                       .await
                       .map_err(Error::Sqlx)?;

                    let mut lower = sqlx::query_as("SELECT * FROM `messages` WHERE `channel_id` = ? AND `id` < ? ORDER BY `timestamp` DESC LIMIT ?")
                        .bind(channel_id)
                        .bind(around_id)
                        .bind(limit)
                        .fetch_all(db)
                        .await
                        .map_err(Error::Sqlx)?;

                    upper.append(&mut lower);
                    upper.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

                    Ok(upper)
                } else {
                    Message::get_by_id(db, channel_id, around_id).await.map(|res| res.map_or(vec![], |msg| vec![msg]))
                }
            },
            ChannelMessagesAnchor::After(after_id) => {
                sqlx::query_as("SELECT * FROM `messages` WHERE `channel_id` = ? AND `id` > ? ORDER BY `timestamp` DESC LIMIT ?")
                    .bind(channel_id)
                    .bind(after_id)
                    .bind(limit)
                    .fetch_all(db)
                    .await
                    .map_err(Error::Sqlx)
            }
        }
	}

	pub async fn get_pinned(db: &PgPool, channel_id: Snowflake) -> Result<Vec<Self>, Error> {
		sqlx::query_as("SELECT * FROM `messages` WHERE `channel_id` = ? AND `pinned` = true")
			.bind(channel_id)
			.fetch_all(db)
			.await
			.map_err(Error::Sqlx)
	}

	pub async fn count_by_user_in_window(
		db: &PgPool,
		channel_id: Snowflake,
		author_id: Snowflake,
		window: u64,
	) -> Result<i32, Error> {
		let window = PgU64::from(window);
		let res = sqlx::query("SELECT COUNT(*) FROM `messages` WHERE `channel_id` = ? AND `author_id` = ? AND `timestamp` > NOW() - INTERVAL ? SECOND")
            .bind(channel_id)
            .bind(author_id)
            .bind(window)
            .fetch_one(db)
            .await?;

		let data = res.get::<i32, _>(0);
		Ok(data)
	}

	pub async fn count_pinned(db: &PgPool, channel_id: Snowflake) -> Result<i32, Error> {
		let res = sqlx::query(
			"SELECT COUNT(*) FROM `messages` WHERE `channel_id` = ? AND `pinned` = true",
		)
		.bind(channel_id)
		.fetch_one(db)
		.await?;

		let data = res.get::<i32, _>(0);
		Ok(data)
	}

	pub async fn count(db: &PgPool) -> Result<i32, Error> {
		sqlx::query("SELECT COUNT(*) FROM `messages`")
			.fetch_one(db)
			.await
			.map_err(Error::from)
			.map(|r| r.get::<i32, _>(0))
	}

	pub async fn populate_relations(&mut self, db: &PgPool) -> Result<(), Error> {
		self.author = User::get_by_id(db, self.author_id).await?.map(|u| u.to_public_user());
		Ok(())
	}

	pub async fn modify(&mut self, db: &PgPool, payload: MessageModifySchema) -> Result<(), Error> {
		if let Some(content) = &payload.content {
			self.content = Some(content.to_owned());
		}
		if let Some(embeds) = &payload.embeds {
			self.embeds = sqlx::types::Json(embeds.to_owned());
		}
		if let Some(components) = &payload.components {
			self.components = Some(sqlx::types::Json(components.to_owned()));
		}
		if let Some(flags) = &payload.flags {
			self.flags = Some(flags.to_owned());
		}
		if let Some(files) = &payload.files {
			// TODO: Handle file uploads
		}

		todo!()
	}

	pub async fn set_pinned(&mut self, db: &PgPool, pinned: bool) -> Result<(), Error> {
		self.pinned = pinned;
		sqlx::query("UPDATE `messages` SET `pinned` = ? WHERE `id` = ?")
			.bind(pinned)
			.bind(self.id)
			.execute(db)
			.await
			.map_err(Error::Sqlx)?;

		Ok(())
	}

	pub async fn clear_reactions(&mut self, db: &PgPool) -> Result<(), Error> {
		self.reactions = None;
		self.save(db).await?;
		Ok(())
	}

	pub async fn remove_reaction(&mut self, db: &PgPool, emoji: PartialEmoji) -> Result<(), Error> {
		if let Some(reactions) = self.reactions.as_mut() {
			let orig_len = reactions.len();
			reactions.retain(|r| {
				if emoji.id.is_some() && r.emoji.id.is_some() {
					r.emoji.id != emoji.id
				} else {
					r.emoji.name.ne(&emoji.name)
				}
			});
			if orig_len == reactions.len() {
				return Err(Error::Reaction(ReactionError::NotFound));
			}
		}
		self.save(db).await?;
		Ok(())
	}

	pub async fn save(&self, db: &PgPool) -> Result<(), Error> {
		sqlx::query("UPDATE `messages` SET `content` = ?, `embeds` = ?, `attachments` = ?, `components` = ?, `flags` = ?, `edited_timestamp` = NOW() WHERE `id` = ?")
            .bind(&self.content)
            .bind(&self.embeds)
            .bind(&self.components)
            .bind(self.flags)
            .execute(db)
            .await
            .map(|_| ())
            .map_err(Error::Sqlx)
	}

	pub async fn delete(&self, db: &PgPool) -> Result<(), Error> {
		sqlx::query("DELETE FROM `messages` WHERE `id` = ?")
			.bind(self.id)
			.execute(db)
			.await
			.map(|_| ())
			.map_err(Error::Sqlx)
	}

	pub async fn bulk_delete(db: &PgPool, ids: Vec<Snowflake>) -> Result<(), Error> {
		// TODO: Limit the timeframe?
		let mut query_builder = QueryBuilder::new("DELETE FROM `messages` WHERE `id` IN (");

		let mut separated = query_builder.separated(", ");
		for id in ids {
			separated.push_bind(id);
		}
		separated.push_unseparated(") ");

		let query = query_builder.build();

		query.execute(db).await?;

		Ok(())
	}

	pub fn get_reaction(&self, emoji: &PartialEmoji) -> Option<&Reaction> {
		if let Some(reactions) = &self.reactions {
			reactions.iter().find(|r| {
				if emoji.id.is_some() && r.emoji.id.is_some() {
					r.emoji.id == emoji.id
				} else {
					r.emoji.name == emoji.name
				}
			})
		} else {
			None
		}
	}

	pub fn get_reaction_mut(&mut self, emoji: &PartialEmoji) -> Option<&mut Reaction> {
		if let Some(reactions) = self.reactions.as_mut() {
			reactions.iter_mut().find(|r| {
				if emoji.id.is_some() && r.emoji.id.is_some() {
					r.emoji.id == emoji.id
				} else {
					r.emoji.name == emoji.name
				}
			})
		} else {
			None
		}
	}

	pub async fn search(
		db: &PgPool,
		guild_id: impl Into<Option<Snowflake>>,
		channel_id: impl Into<Option<Snowflake>>,
		search_payload: &MessageSearchQuery,
	) -> Result<Vec<Self>, Error> {
		let guild_id = guild_id.into();
		let channel_id = channel_id.into();

		let mut query_builder = QueryBuilder::new("SELECT * FROM `messages` WHERE ");
		let where_separated = query_builder.separated(" AND ");

		// if let Some(guild_id) = guild_id {
		//     where_separated.push("guild_id = ").push_bind(guild_id);
		// }
		//
		// if let Some(channel_id) = channel_id {
		//     where_separated.push("channel_id = ").push_bind(channel_id);
		// }
		//
		// if let Some(author_ids) = &search_payload.author_id {
		//     where_separated.push(" author_id IN (");
		//     let mut separated = query_builder.separated(", ");
		//     for author_id in author_ids {
		//         separated.push_bind(author_id);
		//     }
		//     separated.push_unseparated(") ");
		// }
		//
		// // if let Some(author_types) = &search_payload.author_type {
		// //     where_separated.push(" author_type IN (")
		// //     for author_type in author_types {
		// //
		// //     }
		// // }
		//
		// if let Some(channel_ids) = &search_payload.channel_id {
		//     where_separated.push(" channel_id IN (");
		//     let mut separated = query_builder.separated(", ");
		//     for channel_id in channel_ids {
		//         separated.push_bind(channel_id);
		//     }
		//     separated.push_unseparated(") ");
		// }
		//
		// if let Some(command_ids) = &search_payload.command_id {
		//     // TODO:
		// }
		//
		// if let Some(content) = &search_payload.content {
		//     where_separated
		//         .push(" content LIKE ")
		//         .push_bind(format!("%{}%", content));
		// }
		//
		// if let Some(embed_providers) = &search_payload.embed_provider {
		//     // TODO:
		// }
		//
		// if let Some(embed_types) = &search_payload.embed_type {
		//     // TODO:
		// }
		//
		// if let Some(has) = &search_payload.has {
		//     // TODO: Might need a search index table?
		// }

		let query = query_builder.build();

		let res = query.fetch_all(db).await.map_err(Error::Sqlx)?;

		Ok(res.into_iter().flat_map(|r| Message::from_row(&r)).collect::<Vec<_>>())
	}
}
