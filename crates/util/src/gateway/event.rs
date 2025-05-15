// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chorus::types::{GatewayHello, Opcode};
use serde::{Deserialize, Serialize};

use super::{
	dispatchevent::{DispatchEvent, DispatchEventType},
	*,
};

#[derive(
	Debug,
	::serde::Deserialize,
	::serde::Serialize,
	Clone,
	Copy,
	PartialEq,
	PartialOrd,
	Eq,
	Ord,
	Hash,
)]
/// Enum representing all possible event types that can be received from or sent
/// to the gateway.
pub enum EventType {
	Hello,
	Heartbeat,
	Dispatch(DispatchEventType),
	Identify,
	Resume,
	InvalidSession,
	PresenceUpdate,
	VoiceStateUpdate,
	VoiceServerPing,
	Reconnect,
	RequestGuildMembers,
	HeartbeatAck,
	CallConnect,
	GuildSubscriptions,
	LobbyConnect,
	LobbyDisconnect,
	LobbyVoiceStates,
	StreamCreate,
	StreamDelete,
	StreamWatch,
	StreamPing,
	StreamSetPaused,
	EmbeddedActivityCreate,
	EmbeddedActivityUpdate,
	EmbeddedActivityDelete,
	RequestForumUnreads,
	RemoteCommand,
	RequestDeletedEntityIDs,
	RequestSoundboardSounds,
	SpeedTestCreate,
	SpeedTestDelete,
	RequestLastMessages,
	SearchRecentMembers,
	RequestChannelStatuses,
}

impl EventType {
	pub fn op_code(&self) -> Opcode {
		match self {
			Self::Hello => Opcode::Hello,
			Self::Heartbeat => Opcode::Heartbeat,
			Self::Dispatch(_) => Opcode::Dispatch,
			Self::Identify => Opcode::Identify,
			Self::Resume => Opcode::Resume,
			Self::InvalidSession => Opcode::InvalidSession,
			Self::PresenceUpdate => Opcode::PresenceUpdate,
			Self::VoiceStateUpdate => Opcode::VoiceStateUpdate,
			Self::VoiceServerPing => Opcode::VoiceServerPing,
			Self::Reconnect => Opcode::Reconnect,
			Self::RequestGuildMembers => Opcode::RequestGuildMembers,
			Self::HeartbeatAck => Opcode::HeartbeatAck,
			Self::CallConnect => Opcode::CallConnect,
			Self::GuildSubscriptions => Opcode::GuildSubscriptions,
			Self::LobbyConnect => Opcode::LobbyConnect,
			Self::LobbyDisconnect => Opcode::LobbyDisconnect,
			Self::LobbyVoiceStates => Opcode::LobbyVoiceStates,
			Self::StreamCreate => Opcode::StreamCreate,
			Self::StreamDelete => Opcode::StreamDelete,
			Self::StreamWatch => Opcode::StreamWatch,
			Self::StreamPing => Opcode::StreamPing,
			Self::StreamSetPaused => Opcode::StreamSetPaused,
			Self::EmbeddedActivityCreate => Opcode::EmbeddedActivityCreate,
			Self::EmbeddedActivityUpdate => Opcode::EmbeddedActivityUpdate,
			Self::EmbeddedActivityDelete => Opcode::EmbeddedActivityDelete,
			Self::RequestForumUnreads => Opcode::RequestForumUnreads,
			Self::RemoteCommand => Opcode::RemoteCommand,
			Self::RequestDeletedEntityIDs => Opcode::RequestDeletedEntityIDs,
			Self::RequestSoundboardSounds => Opcode::RequestSoundboardSounds,
			Self::SpeedTestCreate => Opcode::SpeedTestCreate,
			Self::SpeedTestDelete => Opcode::SpeedTestDelete,
			Self::RequestLastMessages => Opcode::RequestLastMessages,
			Self::SearchRecentMembers => Opcode::SearchRecentMembers,
			Self::RequestChannelStatuses => Opcode::RequestChannelStatuses,
		}
	}
}

impl From<EventType> for Opcode {
	fn from(event_type: EventType) -> Self {
		event_type.op_code()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// This enum is supposed to represent all possible event types/payloads that
/// can be received from or sent to the gateway.
///
/// ## Incompleteness Warning
///
/// The types `T` in `GatewayPayload<T>` might not yet be correct or complete
/// for all events. Please feel free to file a PR or an issue should you find
/// any discrepancies.
#[serde(untagged)]
pub enum Event {
	Hello(GatewayHello),
	Heartbeat(GatewayHeartbeat),
	Dispatch(DispatchEvent),
	Identify(GatewayPayload<GatewayIdentifyPayload>),
	Resume(GatewayPayload<GatewayResume>),
	InvalidSession(GatewayPayload<GatewayInvalidSession>),
	PresenceUpdate(GatewayPayload<PresenceUpdate>),
	VoiceStateUpdate(GatewayPayload<VoiceStateUpdate>),
	VoiceServerPing(GatewayPayload<VoiceServerUpdate>),
	Reconnect(GatewayPayload<()>),
	RequestGuildMembers(GatewayPayload<GatewayRequestGuildMembers>),
	HeartbeatAck(GatewayPayload<GatewayHeartbeatAck>),
	CallConnect(GatewayPayload<()>),
	GuildSubscriptions(GatewayPayload<()>),
	LobbyConnect(GatewayPayload<()>),
	LobbyDisconnect(GatewayPayload<()>),
	LobbyVoiceStates(GatewayPayload<()>),
	StreamCreate(GatewayPayload<()>),
	StreamDelete(GatewayPayload<()>),
	StreamWatch(GatewayPayload<()>),
	StreamPing(GatewayPayload<()>),
	StreamSetPaused(GatewayPayload<()>),
	EmbeddedActivityCreate(GatewayPayload<()>),
	EmbeddedActivityUpdate(GatewayPayload<()>),
	EmbeddedActivityDelete(GatewayPayload<()>),
	RequestForumUnreads(GatewayPayload<()>),
	RemoteCommand(GatewayPayload<()>),
	RequestDeletedEntityIDs(GatewayPayload<()>),
	RequestSoundboardSounds(GatewayPayload<()>),
	SpeedTestCreate(GatewayPayload<()>),
	SpeedTestDelete(GatewayPayload<()>),
	RequestLastMessages(GatewayPayload<()>),
	SearchRecentMembers(GatewayPayload<()>),
	RequestChannelStatuses(GatewayPayload<()>),
}

impl Event {
	pub fn op_code(&self) -> Opcode {
		match self {
			Event::Hello(gateway_hello) => Opcode::Hello,
			Event::Heartbeat(gateway_heartbeat) => Opcode::Heartbeat,
			Event::Dispatch(dispatch_event) => Opcode::Dispatch,
			Event::Identify(gateway_payload) => Opcode::Identify,
			Event::Resume(gateway_payload) => Opcode::Resume,
			Event::InvalidSession(gateway_payload) => Opcode::InvalidSession,
			Event::PresenceUpdate(gateway_payload) => Opcode::PresenceUpdate,
			Event::VoiceStateUpdate(gateway_payload) => Opcode::VoiceStateUpdate,
			Event::VoiceServerPing(gateway_payload) => Opcode::VoiceServerPing,
			Event::Reconnect(gateway_payload) => Opcode::Reconnect,
			Event::RequestGuildMembers(gateway_payload) => Opcode::RequestGuildMembers,
			Event::HeartbeatAck(gateway_payload) => Opcode::HeartbeatAck,
			Event::CallConnect(gateway_payload) => Opcode::CallConnect,
			Event::GuildSubscriptions(gateway_payload) => Opcode::GuildSubscriptions,
			Event::LobbyConnect(gateway_payload) => Opcode::LobbyConnect,
			Event::LobbyDisconnect(gateway_payload) => Opcode::LobbyDisconnect,
			Event::LobbyVoiceStates(gateway_payload) => Opcode::LobbyVoiceStates,
			Event::StreamCreate(gateway_payload) => Opcode::StreamCreate,
			Event::StreamDelete(gateway_payload) => Opcode::StreamDelete,
			Event::StreamWatch(gateway_payload) => Opcode::StreamWatch,
			Event::StreamPing(gateway_payload) => Opcode::StreamPing,
			Event::StreamSetPaused(gateway_payload) => Opcode::StreamSetPaused,
			Event::EmbeddedActivityCreate(gateway_payload) => Opcode::EmbeddedActivityCreate,
			Event::EmbeddedActivityUpdate(gateway_payload) => Opcode::EmbeddedActivityUpdate,
			Event::EmbeddedActivityDelete(gateway_payload) => Opcode::EmbeddedActivityDelete,
			Event::RequestForumUnreads(gateway_payload) => Opcode::RequestForumUnreads,
			Event::RemoteCommand(gateway_payload) => Opcode::RemoteCommand,
			Event::RequestDeletedEntityIDs(gateway_payload) => Opcode::RequestDeletedEntityIDs,
			Event::RequestSoundboardSounds(gateway_payload) => Opcode::RequestSoundboardSounds,
			Event::SpeedTestCreate(gateway_payload) => Opcode::SpeedTestCreate,
			Event::SpeedTestDelete(gateway_payload) => Opcode::SpeedTestDelete,
			Event::RequestLastMessages(gateway_payload) => Opcode::RequestLastMessages,
			Event::SearchRecentMembers(gateway_payload) => Opcode::SearchRecentMembers,
			Event::RequestChannelStatuses(gateway_payload) => Opcode::RequestChannelStatuses,
		}
	}
}

impl From<Event> for u8 {
	fn from(value: Event) -> Self {
		value.op_code() as u8
	}
}

/// Equivalent to
///
/// ```rust
/// Ok(Event::Heartbeat(from_str(&message)?))
/// ```
macro_rules! convert_to {
	($event_variant:path, $message:expr) => {
		Ok($event_variant(from_str(&$message)?))
	};
}

impl TryFrom<tokio_tungstenite::tungstenite::Message> for Event {
	type Error = Error;

	fn try_from(message: tokio_tungstenite::tungstenite::Message) -> Result<Self, Self::Error> {
		/// Takes a message of unknown type as input and tries to convert it to
		/// an [Event].
		let message_as_string = message.to_string();
		// Payload type of option string is okay, since raw_gateway_payload is only used
		// to look at the opcode and, if the opcode is 0 (= dispatch), the event name
		// in the received message
		let raw_gateway_payload: GatewayPayload<Option<serde_json::Value>> =
			from_str(&message_as_string)?;
		match Opcode::try_from(raw_gateway_payload.op_code).map_err(|_| {
			Error::Gateway(GatewayError::UnexpectedOpcode(raw_gateway_payload.op_code.into()))
		})? {
			Opcode::Heartbeat => return convert_to!(Event::Heartbeat, message_as_string),
			Opcode::Identify => return convert_to!(Event::Identify, message_as_string),
			Opcode::PresenceUpdate => return convert_to!(Event::PresenceUpdate, message_as_string),
			Opcode::VoiceStateUpdate => {
				return convert_to!(Event::VoiceStateUpdate, message_as_string);
			}
			Opcode::VoiceServerPing => {
				return convert_to!(Event::VoiceServerPing, message_as_string);
			}
			Opcode::Resume => return convert_to!(Event::Resume, message_as_string),
			Opcode::Reconnect => return convert_to!(Event::Reconnect, message_as_string),
			Opcode::RequestGuildMembers => {
				return convert_to!(Event::RequestGuildMembers, message_as_string);
			}
			Opcode::InvalidSession => return convert_to!(Event::InvalidSession, message_as_string),
			Opcode::Hello => return convert_to!(Event::Hello, message_as_string),
			Opcode::HeartbeatAck => return convert_to!(Event::HeartbeatAck, message_as_string),
			#[allow(deprecated)]
			Opcode::GuildSync => {
				return Err(Error::Gateway(GatewayError::UnexpectedMessage(format!(
					"Deprecated opcode: {}",
					raw_gateway_payload.op_code
				))));
			}
			Opcode::CallConnect => return convert_to!(Event::CallConnect, message_as_string),
			Opcode::GuildSubscriptions => {
				return convert_to!(Event::GuildSubscriptions, message_as_string);
			}
			Opcode::LobbyConnect => return convert_to!(Event::LobbyConnect, message_as_string),
			Opcode::LobbyDisconnect => {
				return convert_to!(Event::LobbyDisconnect, message_as_string);
			}
			Opcode::LobbyVoiceStates => {
				return convert_to!(Event::LobbyVoiceStates, message_as_string);
			}
			Opcode::StreamCreate => return convert_to!(Event::StreamCreate, message_as_string),
			Opcode::StreamDelete => return convert_to!(Event::StreamDelete, message_as_string),
			Opcode::StreamWatch => return convert_to!(Event::StreamWatch, message_as_string),
			Opcode::StreamPing => return convert_to!(Event::StreamPing, message_as_string),
			Opcode::StreamSetPaused => {
				return convert_to!(Event::StreamSetPaused, message_as_string);
			}
			#[allow(deprecated)]
			Opcode::LfgSubscriptions => {
				return Err(Error::Gateway(GatewayError::UnexpectedMessage(format!(
					"Deprecated opcode {} will not be processed",
					raw_gateway_payload.op_code
				))));
			}
			#[allow(deprecated)]
			Opcode::RequestGuildApplicationCommands => {
				return Err(Error::Gateway(GatewayError::UnexpectedMessage(format!(
					"Deprecated opcode {} will not be processed",
					raw_gateway_payload.op_code
				))));
			}
			Opcode::EmbeddedActivityCreate => {
				return convert_to!(Event::EmbeddedActivityCreate, message_as_string);
			}
			Opcode::EmbeddedActivityDelete => {
				return convert_to!(Event::EmbeddedActivityDelete, message_as_string);
			}
			Opcode::EmbeddedActivityUpdate => {
				return convert_to!(Event::EmbeddedActivityUpdate, message_as_string);
			}
			Opcode::RequestForumUnreads => {
				return convert_to!(Event::RequestForumUnreads, message_as_string);
			}
			Opcode::RemoteCommand => return convert_to!(Event::RemoteCommand, message_as_string),
			Opcode::RequestDeletedEntityIDs => {
				return convert_to!(Event::RequestDeletedEntityIDs, message_as_string);
			}
			Opcode::RequestSoundboardSounds => {
				return convert_to!(Event::RequestSoundboardSounds, message_as_string);
			}
			Opcode::SpeedTestCreate => {
				return convert_to!(Event::SpeedTestCreate, message_as_string);
			}
			Opcode::SpeedTestDelete => {
				return convert_to!(Event::SpeedTestDelete, message_as_string);
			}
			Opcode::RequestLastMessages => {
				return convert_to!(Event::RequestLastMessages, message_as_string);
			}
			Opcode::SearchRecentMembers => {
				return convert_to!(Event::SearchRecentMembers, message_as_string);
			}
			Opcode::RequestChannelStatuses => {
				return convert_to!(Event::RequestChannelStatuses, message_as_string);
			}
			// Dispatch has to be handled differently. To not nest further, we just do nothing here,
			// then handle it outside of this
			Opcode::Dispatch => (),
			o => {
				return Err(GatewayError::UnexpectedMessage(format!(
					"Opcode not implemented: {}",
					o as u8
				))
				.into());
			}
		};

		let dispatch_event_name = match raw_gateway_payload.event_name {
			Some(n) => n,
			None => {
				return Err(GatewayError::UnexpectedMessage(format!(
					"No event name provided on dispatch event: {}",
					message_as_string
				))
				.into());
			}
		};

		let dispatch_event_type = DispatchEventType::try_from(dispatch_event_name.as_str())
			.map_err(|_| {
				GatewayError::UnexpectedMessage(format!(
					"Unknown dispatch event: {}",
					dispatch_event_name
				))
			})?;

		// At this point we know what Dispatch event we are dealing with and can try to
		// deserialize it

		match dispatch_event_type {
			DispatchEventType::Ready => {
				convert_to!(DispatchEvent::Ready, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::ReadySupplemental => {
				convert_to!(DispatchEvent::ReadySupplemental, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::Resumed => {
				convert_to!(DispatchEvent::Resumed, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::AuthSessionChange => {
				convert_to!(DispatchEvent::AuthSessionChange, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::AuthenticatorCreate => {
				convert_to!(DispatchEvent::AuthenticatorCreate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::AuthenticatorUpdate => {
				convert_to!(DispatchEvent::AuthenticatorUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::AuthenticatorDelete => {
				convert_to!(DispatchEvent::AuthenticatorDelete, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::ApplicationCommandPermissionsUpdate => {
				convert_to!(DispatchEvent::ApplicationCommandPermissionsUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::AutoModerationRuleCreate => {
				convert_to!(DispatchEvent::AutoModerationRuleCreate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::AutoModerationRuleUpdate => {
				convert_to!(DispatchEvent::AutoModerationRuleUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::AutoModerationRuleDelete => {
				convert_to!(DispatchEvent::AutoModerationRuleDelete, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::AutoModerationActionExecution => {
				convert_to!(DispatchEvent::AutoModerationActionExecution, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::AutoModerationMentionRaidDetection => {
				convert_to!(DispatchEvent::AutoModerationMentionRaidDetection, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::CallCreate => {
				convert_to!(DispatchEvent::CallCreate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::CallUpdate => {
				convert_to!(DispatchEvent::CallUpdate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::CallDelete => {
				convert_to!(DispatchEvent::CallDelete, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::ChannelCreate => {
				convert_to!(DispatchEvent::ChannelCreate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::ChannelUpdate => {
				convert_to!(DispatchEvent::ChannelUpdate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::ChannelDelete => {
				convert_to!(DispatchEvent::ChannelDelete, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::ChannelStatuses => {
				convert_to!(DispatchEvent::ChannelStatuses, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::VoiceChannelStatusUpdate => {
				convert_to!(DispatchEvent::VoiceChannelStatusUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::ChannelPinsUpdate => {
				convert_to!(DispatchEvent::ChannelPinsUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::ChannelRecipientAdd => {
				convert_to!(DispatchEvent::ChannelRecipientAdd, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::ChannelRecipientRemove => {
				convert_to!(DispatchEvent::ChannelRecipientRemove, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::DmSettingsUpsellShow => {
				convert_to!(DispatchEvent::DmSettingsUpsellShow, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::ThreadCreate => {
				convert_to!(DispatchEvent::ThreadCreate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::ThreadUpdate => {
				convert_to!(DispatchEvent::ThreadUpdate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::ThreadDelete => {
				convert_to!(DispatchEvent::ThreadDelete, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::ThreadListSync => {
				convert_to!(DispatchEvent::ThreadListSync, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::ThreadMemberUpdate => {
				convert_to!(DispatchEvent::ThreadMemberUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::ThreadMembersUpdate => {
				convert_to!(DispatchEvent::ThreadMembersUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::FriendSuggestionCreate => {
				convert_to!(DispatchEvent::FriendSuggestionCreate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::FriendSuggestionDelete => {
				convert_to!(DispatchEvent::FriendSuggestionDelete, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildCreate => {
				convert_to!(DispatchEvent::GuildCreate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::GuildUpdate => {
				convert_to!(DispatchEvent::GuildUpdate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::GuildDelete => {
				convert_to!(DispatchEvent::GuildDelete, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::GuildAuditLogEntryCreate => {
				convert_to!(DispatchEvent::GuildAuditLogEntryCreate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildBanAdd => {
				convert_to!(DispatchEvent::GuildBanAdd, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::GuildBanRemove => {
				convert_to!(DispatchEvent::GuildBanRemove, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::GuildEmojisUpdate => {
				convert_to!(DispatchEvent::GuildEmojisUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildStickersUpdate => {
				convert_to!(DispatchEvent::GuildStickersUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildJoinRequestCreate => {
				convert_to!(DispatchEvent::GuildJoinRequestCreate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildJoinRequestUpdate => {
				convert_to!(DispatchEvent::GuildJoinRequestUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildJoinRequestDelete => {
				convert_to!(DispatchEvent::GuildJoinRequestDelete, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildMemberAdd => {
				convert_to!(DispatchEvent::GuildMemberAdd, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::GuildMemberRemove => {
				convert_to!(DispatchEvent::GuildMemberRemove, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildMemberUpdate => {
				convert_to!(DispatchEvent::GuildMemberUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildMembersChunk => {
				convert_to!(DispatchEvent::GuildMembersChunk, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildRoleCreate => {
				convert_to!(DispatchEvent::GuildRoleCreate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::GuildRoleUpdate => {
				convert_to!(DispatchEvent::GuildRoleUpdate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::GuildRoleDelete => {
				convert_to!(DispatchEvent::GuildRoleDelete, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::GuildScheduledEventCreate => {
				convert_to!(DispatchEvent::GuildScheduledEventCreate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildScheduledEventUpdate => {
				convert_to!(DispatchEvent::GuildScheduledEventUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildScheduledEventDelete => {
				convert_to!(DispatchEvent::GuildScheduledEventDelete, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildScheduledEventUserAdd => {
				convert_to!(DispatchEvent::GuildScheduledEventUserAdd, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildScheduledEventUserRemove => {
				convert_to!(DispatchEvent::GuildScheduledEventUserRemove, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildSoundboardSoundCreate => {
				convert_to!(DispatchEvent::GuildSoundboardSoundCreate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildSoundboardSoundUpdate => {
				convert_to!(DispatchEvent::GuildSoundboardSoundUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::GuildSoundboardSoundDelete => {
				convert_to!(DispatchEvent::GuildSoundboardSoundDelete, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::SoundboardSounds => {
				convert_to!(DispatchEvent::SoundboardSounds, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::GuildIntegrationsUpdate => {
				convert_to!(DispatchEvent::GuildIntegrationsUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::IntegrationCreate => {
				convert_to!(DispatchEvent::IntegrationCreate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::IntegrationUpdate => {
				convert_to!(DispatchEvent::IntegrationUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::IntegrationDelete => {
				convert_to!(DispatchEvent::IntegrationDelete, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::InteractionCreate => {
				convert_to!(DispatchEvent::InteractionCreate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::InviteCreate => {
				convert_to!(DispatchEvent::InviteCreate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::InviteDelete => {
				convert_to!(DispatchEvent::InviteDelete, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::MessageCreate => {
				convert_to!(DispatchEvent::MessageCreate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::MessageUpdate => {
				convert_to!(DispatchEvent::MessageUpdate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::MessageDelete => {
				convert_to!(DispatchEvent::MessageDelete, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::MessageDeleteBulk => {
				convert_to!(DispatchEvent::MessageDeleteBulk, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::MessagePollVoteAdd => {
				convert_to!(DispatchEvent::MessagePollVoteAdd, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::MessagePollVoteRemove => {
				convert_to!(DispatchEvent::MessagePollVoteRemove, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::MessageReactionAdd => {
				convert_to!(DispatchEvent::MessageReactionAdd, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::MessageReactionAddMany => {
				convert_to!(DispatchEvent::MessageReactionAddMany, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::MessageReactionRemove => {
				convert_to!(DispatchEvent::MessageReactionRemove, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::MessageReactionRemoveAll => {
				convert_to!(DispatchEvent::MessageReactionRemoveAll, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::MessageReactionRemoveEmoji => {
				convert_to!(DispatchEvent::MessageReactionRemoveEmoji, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::RecentMentionDelete => {
				convert_to!(DispatchEvent::RecentMentionDelete, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::LastMessages => {
				convert_to!(DispatchEvent::LastMessages, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::Oauth2TokenRevoke => {
				convert_to!(DispatchEvent::Oauth2TokenRevoke, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::PresenceUpdate => {
				convert_to!(DispatchEvent::PresenceUpdate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::RelationshipAdd => {
				convert_to!(DispatchEvent::RelationshipAdd, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::RelationshipUpdate => {
				convert_to!(DispatchEvent::RelationshipUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::RelationshipRemove => {
				convert_to!(DispatchEvent::RelationshipRemove, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::StageInstanceCreate => {
				convert_to!(DispatchEvent::StageInstanceCreate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::StageInstanceUpdate => {
				convert_to!(DispatchEvent::StageInstanceUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::StageInstanceDelete => {
				convert_to!(DispatchEvent::StageInstanceDelete, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::TypingStart => {
				convert_to!(DispatchEvent::TypingStart, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::UserUpdate => {
				convert_to!(DispatchEvent::UserUpdate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::UserApplicationRemove => {
				convert_to!(DispatchEvent::UserApplicationRemove, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::UserConnectionsUpdate => {
				convert_to!(DispatchEvent::UserConnectionsUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::UserNoteUpdate => {
				convert_to!(DispatchEvent::UserNoteUpdate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::UserRequiredActionUpdate => {
				convert_to!(DispatchEvent::UserRequiredActionUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::UserSettingsUpdate => {
				convert_to!(DispatchEvent::UserSettingsUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::VoiceStateUpdate => {
				convert_to!(DispatchEvent::VoiceStateUpdate, message_as_string).map(Event::Dispatch)
			}
			DispatchEventType::VoiceServerUpdate => {
				convert_to!(DispatchEvent::VoiceServerUpdate, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::VoiceChannelEffectSend => {
				convert_to!(DispatchEvent::VoiceChannelEffectSend, message_as_string)
					.map(Event::Dispatch)
			}
			DispatchEventType::WebhooksUpdate => {
				convert_to!(DispatchEvent::WebhooksUpdate, message_as_string).map(Event::Dispatch)
			}
		}
	}
}

#[cfg(test)]
mod tests {

	use serde_json::Value;

	use super::*;
	#[test]
	fn identify_from_raw_json() {
		let json = r#"{"op":2,"d":{"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3Mjk4Njg3MzQsImlhdCI6MTcyOTc4MjMzNCwiZW1haWwiOiJkZmdkc2Znc2RmZ0Bkc2Zmc2Rmc2QuZGUiLCJpZCI6IjEyOTkwMjYwMDU1MzIzNDg0MTYifQ.3mFo83e0ehI4JWUFy631hUXPJKxjJWUSIT5laDTbzzU","capabilities":16381,"properties":{"browser":"Spacebar Web","client_build_number":0,"release_channel":"dev","browser_user_agent":"Mozilla/5.0 (X11; Linux x86_64; rv:131.0) Gecko/20100101 Firefox/131.0"},"compress":false,"presence":{"status":"online","since":1729782873344,"activities":[],"afk":false}}}"#;
		let message = Message::Text(json.to_string().into());
		let gateway_payload_string =
			from_str::<GatewayPayload<Option<Value>>>(&message.to_string()).unwrap();
		dbg!(gateway_payload_string);
		let event = Event::try_from(message).unwrap();
		dbg!(event);
	}

	#[test]
	fn heartbeat_from_raw_json() {
		let json = r#"{"op":1}"#;
		let message = Message::Text(json.to_string().into());
		let gateway_payload_string =
			from_str::<GatewayPayload<Option<Value>>>(&message.to_string()).unwrap();
		dbg!(gateway_payload_string);
		let event = Event::try_from(message).unwrap();
		dbg!(event);
	}
}
