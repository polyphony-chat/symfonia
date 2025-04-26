// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod dispatchevent;
pub mod event;

use std::{
	collections::{HashMap, HashSet},
	fmt::Display,
	ops::{Deref, DerefMut},
	sync::{Arc, Weak},
};

use ::serde::{Deserialize, Serialize, de::DeserializeOwned};
use chorus::types::{
	ChannelCreate, ChannelDelete, ChannelUpdate, GatewayHeartbeat, GatewayHeartbeatAck,
	GatewayHello, GatewayIdentifyPayload, GatewayInvalidSession, GatewayReady,
	GatewayReadySupplemental, GatewayRequestGuildMembers, GatewayResume, GuildBanAdd,
	GuildBanRemove, GuildCreate, GuildDelete, GuildEmojisUpdate, GuildIntegrationsUpdate,
	GuildMemberAdd, GuildMemberRemove, GuildMemberUpdate, GuildMembersChunk, GuildUpdate,
	InteractionCreate, InviteCreate, InviteDelete, MessageCreate, MessageDelete, MessageDeleteBulk,
	MessageReactionAdd, MessageReactionRemove, MessageReactionRemoveAll,
	MessageReactionRemoveEmoji, MessageUpdate, Opcode, PresenceUpdate, Snowflake,
	StageInstanceCreate, StageInstanceDelete, StageInstanceUpdate, ThreadCreate, ThreadDelete,
	ThreadListSync, ThreadMemberUpdate, ThreadMembersUpdate, ThreadUpdate, TypingStartEvent,
	UserUpdate, VoiceServerUpdate, VoiceStateUpdate, WebhooksUpdate,
};
pub use dispatchevent::*;
pub use event::*;
use futures::{
	SinkExt, StreamExt,
	stream::{SplitSink, SplitStream},
};
use log::log;
use pubserve::Subscriber;
use serde_json::from_str;
use sqlx::PgPool;
use sqlx_pg_uint::PgU64;
use tokio::{
	net::TcpStream,
	sync::{Mutex, RwLock},
};
use tokio_tungstenite::{
	WebSocketStream,
	tungstenite::{
		Message,
		protocol::{CloseFrame, frame::coding::CloseCode},
	},
};

#[derive(Serialize, Clone, PartialEq, Debug)]
/// A de-/serializable data payload for transmission over the gateway.
pub struct GatewayPayload<T>
where
	T: Serialize + DeserializeOwned,
{
	#[serde(rename = "op")]
	pub op_code: u8,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "d")]
	pub event_data: Option<T>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "s")]
	pub sequence_number: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "t")]
	pub event_name: Option<String>,
}

impl<T: Serialize + DeserializeOwned> GatewayPayload<T> {
	pub fn has_data(&self) -> bool {
		self.event_data.is_some()
	}

	pub fn has_sequence(&self) -> bool {
		self.sequence_number.is_some()
	}

	pub fn has_event_name(&self) -> bool {
		self.event_name.is_some()
	}
}

impl<'de, T: DeserializeOwned + Serialize> Deserialize<'de> for GatewayPayload<T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: ::serde::Deserializer<'de>,
	{
		let value = serde_json::Value::deserialize(deserializer)?;
		let op_code = value["op"].as_u64().unwrap() as u8;
		let event_data = match value.get("d").cloned() {
			Some(data) => match serde_json::from_value(data) {
				Ok(t) => t,
				Err(e) => return Err(::serde::de::Error::custom(e)),
			},
			None => None,
		};
		let sequence_number = value.get("s").cloned().map(|v| v.as_u64().unwrap());
		let event_name = match value.get("t") {
			Some(v) => v.as_str().map(|v_str| v_str.to_string()),
			None => None,
		};
		Ok(GatewayPayload { op_code, event_data, sequence_number, event_name })
	}
}
