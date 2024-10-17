// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::Display;
use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
    sync::{Arc, Weak},
};

use ::serde::{de::DeserializeOwned, Deserialize, Serialize};
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
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::log;
use parking_lot::RwLock;
use pubserve::Subscriber;
use sqlx::PgPool;
use sqlx_pg_uint::PgU64;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{
    tungstenite::{
        protocol::{frame::coding::CloseCode, CloseFrame},
        Message,
    },
    WebSocketStream,
};

use crate::errors::Error;
use crate::{WebSocketReceive, WebSocketSend};

use super::ResumableClientsStore;

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
/// Enum representing all possible event types that can be received from or sent to the gateway.
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
/// This enum is supposed to represent all possible event types/payloads that can be received from
/// or sent to the gateway.
///
/// ## Incompleteness Warning
///
/// The types `T` in `GatewayPayload<T>` might not yet be correct or complete for all events. Please
/// feel free to file a PR or an issue should you find any discrepancies.
#[serde(rename_all = "PascalCase")]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
/// This enum is supposed to represent all possible dispatch events that can be received from or sent to the
/// gateway. If a variant is missing, it might just be because we haven't caught it yet.
#[serde(rename_all = "PascalCase")]
pub enum DispatchEvent {
    Ready(GatewayPayload<GatewayReady>),
    ReadySupplemental(GatewayPayload<GatewayReadySupplemental>),
    Resumed(GatewayPayload<()>),
    AuthSessionChange(GatewayPayload<()>),
    AuthenticatorCreate(GatewayPayload<()>),
    AuthenticatorUpdate(GatewayPayload<()>),
    AuthenticatorDelete(GatewayPayload<()>),
    ApplicationCommandPermissionsUpdate(GatewayPayload<()>),
    AutoModerationRuleCreate(GatewayPayload<()>),
    AutoModerationRuleUpdate(GatewayPayload<()>),
    AutoModerationRuleDelete(GatewayPayload<()>),
    AutoModerationActionExecution(GatewayPayload<()>),
    AutoModerationMentionRaidDetection(GatewayPayload<()>),
    CallCreate(GatewayPayload<()>),
    CallUpdate(GatewayPayload<()>),
    CallDelete(GatewayPayload<()>),
    ChannelCreate(GatewayPayload<ChannelCreate>),
    ChannelUpdate(GatewayPayload<ChannelUpdate>),
    ChannelDelete(GatewayPayload<ChannelDelete>),
    ChannelStatuses(GatewayPayload<()>),
    VoiceChannelStatusUpdate(GatewayPayload<()>),
    ChannelPinsUpdate(GatewayPayload<()>),
    ChannelRecipientAdd(GatewayPayload<()>),
    ChannelRecipientRemove(GatewayPayload<()>),
    DMSettingsUpsellShow(GatewayPayload<()>),
    ThreadCreate(GatewayPayload<ThreadCreate>),
    ThreadUpdate(GatewayPayload<ThreadUpdate>),
    ThreadDelete(GatewayPayload<ThreadDelete>),
    ThreadListSync(GatewayPayload<ThreadListSync>),
    ThreadMemberUpdate(GatewayPayload<ThreadMemberUpdate>),
    ThreadMembersUpdate(GatewayPayload<ThreadMembersUpdate>),
    FriendSuggestionCreate(GatewayPayload<()>),
    FriendSuggestionDelete(GatewayPayload<()>),
    GuildCreate(GatewayPayload<GuildCreate>),
    GuildUpdate(GatewayPayload<GuildUpdate>),
    GuildDelete(GatewayPayload<GuildDelete>),
    GuildAuditLogEntryCreate(GatewayPayload<()>),
    GuildBanAdd(GatewayPayload<GuildBanAdd>),
    GuildBanRemove(GatewayPayload<GuildBanRemove>),
    GuildEmojisUpdate(GatewayPayload<GuildEmojisUpdate>),
    GuildStickersUpdate(GatewayPayload<()>),
    GuildJoinRequestCreate(GatewayPayload<()>),
    GuildJoinRequestUpdate(GatewayPayload<()>),
    GuildJoinRequestDelete(GatewayPayload<()>),
    GuildMemberAdd(GatewayPayload<GuildMemberAdd>),
    GuildMemberRemove(GatewayPayload<GuildMemberRemove>),
    GuildMemberUpdate(GatewayPayload<GuildMemberUpdate>),
    GuildMembersChunk(GatewayPayload<GuildMembersChunk>),
    GuildMembersRequest(GatewayPayload<GatewayRequestGuildMembers>),
    GuildRoleCreate(GatewayPayload<()>),
    GuildRoleUpdate(GatewayPayload<()>),
    GuildRoleDelete(GatewayPayload<()>),
    GuildScheduledEventCreate(GatewayPayload<()>),
    GuildScheduledEventUpdate(GatewayPayload<()>),
    GuildScheduledEventDelete(GatewayPayload<()>),
    GuildScheduledEventUserAdd(GatewayPayload<()>),
    GuildScheduledEventUserRemove(GatewayPayload<()>),
    GuildSoundboardSoundCreate(GatewayPayload<()>),
    GuildSoundboardSoundUpdate(GatewayPayload<()>),
    GuildSoundboardSoundDelete(GatewayPayload<()>),
    SoundboardSounds(GatewayPayload<()>),
    GuildIntegrationsUpdate(GatewayPayload<GuildIntegrationsUpdate>),
    IntegrationCreate(GatewayPayload<()>),
    IntegrationUpdate(GatewayPayload<()>),
    IntegrationDelete(GatewayPayload<()>),
    InteractionCreate(GatewayPayload<InteractionCreate>),
    InviteCreate(GatewayPayload<InviteCreate>),
    InviteDelete(GatewayPayload<InviteDelete>),
    MessageCreate(GatewayPayload<MessageCreate>),
    MessageUpdate(GatewayPayload<MessageUpdate>),
    MessageDelete(GatewayPayload<MessageDelete>),
    MessageDeleteBulk(GatewayPayload<MessageDeleteBulk>),
    MessagePollVoteAdd(GatewayPayload<()>),
    MessagePollVoteRemove(GatewayPayload<()>),
    MessageReactionAdd(GatewayPayload<MessageReactionAdd>),
    MessageReactionAddMany(GatewayPayload<()>),
    MessageReactionRemove(GatewayPayload<MessageReactionRemove>),
    MessageReactionRemoveAll(GatewayPayload<MessageReactionRemoveAll>),
    MessageReactionRemoveEmoji(GatewayPayload<MessageReactionRemoveEmoji>),
    RecentMentionDelete(GatewayPayload<()>),
    LastMessages(GatewayPayload<()>),
    OAuth2TokenRevoke(GatewayPayload<()>),
    PresenceUpdate(GatewayPayload<PresenceUpdate>),
    RelationshipAdd(GatewayPayload<()>),
    RelationshipUpdate(GatewayPayload<()>),
    RelationshipRemove(GatewayPayload<()>),
    StageInstanceCreate(GatewayPayload<StageInstanceCreate>),
    StageInstanceUpdate(GatewayPayload<StageInstanceUpdate>),
    StageInstanceDelete(GatewayPayload<StageInstanceDelete>),
    TypingStart(GatewayPayload<TypingStartEvent>),
    UserUpdate(GatewayPayload<UserUpdate>),
    UserApplicationRemove(GatewayPayload<()>),
    UserConnectionsUpdate(GatewayPayload<()>),
    UserNoteUpdate(GatewayPayload<()>),
    UserRequiredActionUpdate(GatewayPayload<()>),
    UserSettingsUpdate(GatewayPayload<()>),
    VoiceStateUpdate(GatewayPayload<VoiceStateUpdate>),
    VoiceServerUpdate(GatewayPayload<VoiceServerUpdate>),
    VoiceChannelEffectSend(GatewayPayload<()>),
    WebhooksUpdate(GatewayPayload<WebhooksUpdate>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// This enum is supposed to represent all possible dispatch events that can be received from or sent to the
/// gateway. If a variant is missing, it might just be because we haven't caught it yet.
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DispatchEventType {
    Ready,
    ReadySupplemental,
    Resumed,
    AuthSessionChange,
    AuthenticatorCreate,
    AuthenticatorUpdate,
    AuthenticatorDelete,
    ApplicationCommandPermissionsUpdate,
    AutoModerationRuleCreate,
    AutoModerationRuleUpdate,
    AutoModerationRuleDelete,
    AutoModerationActionExecution,
    AutoModerationMentionRaidDetection,
    CallCreate,
    CallUpdate,
    CallDelete,
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    ChannelStatuses,
    VoiceChannelStatusUpdate,
    ChannelPinsUpdate,
    ChannelRecipientAdd,
    ChannelRecipientRemove,
    DmSettingsUpsellShow,
    ThreadCreate,
    ThreadUpdate,
    ThreadDelete,
    ThreadListSync,
    ThreadMemberUpdate,
    ThreadMembersUpdate,
    FriendSuggestionCreate,
    FriendSuggestionDelete,
    GuildCreate,
    GuildUpdate,
    GuildDelete,
    GuildAuditLogEntryCreate,
    GuildBanAdd,
    GuildBanRemove,
    GuildEmojisUpdate,
    GuildStickersUpdate,
    GuildJoinRequestCreate,
    GuildJoinRequestUpdate,
    GuildJoinRequestDelete,
    GuildMemberAdd,
    GuildMemberRemove,
    GuildMemberUpdate,
    GuildMembersChunk,
    GuildRoleCreate,
    GuildRoleUpdate,
    GuildRoleDelete,
    GuildScheduledEventCreate,
    GuildScheduledEventUpdate,
    GuildScheduledEventDelete,
    GuildScheduledEventUserAdd,
    GuildScheduledEventUserRemove,
    GuildSoundboardSoundCreate,
    GuildSoundboardSoundUpdate,
    GuildSoundboardSoundDelete,
    SoundboardSounds,
    GuildIntegrationsUpdate,
    IntegrationCreate,
    IntegrationUpdate,
    IntegrationDelete,
    InteractionCreate,
    InviteCreate,
    InviteDelete,
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    MessageDeleteBulk,
    MessagePollVoteAdd,
    MessagePollVoteRemove,
    MessageReactionAdd,
    MessageReactionAddMany,
    MessageReactionRemove,
    MessageReactionRemoveAll,
    MessageReactionRemoveEmoji,
    RecentMentionDelete,
    LastMessages,
    Oauth2TokenRevoke,
    PresenceUpdate,
    RelationshipAdd,
    RelationshipUpdate,
    RelationshipRemove,
    StageInstanceCreate,
    StageInstanceUpdate,
    StageInstanceDelete,
    TypingStart,
    UserUpdate,
    UserApplicationRemove,
    UserConnectionsUpdate,
    UserNoteUpdate,
    UserRequiredActionUpdate,
    UserSettingsUpdate,
    VoiceStateUpdate,
    VoiceServerUpdate,
    VoiceChannelEffectSend,
    WebhooksUpdate,
}

impl std::fmt::Display for DispatchEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // use serde to serialize the enum variant to a SCRERAMING_SNAKE_CASE string
        let serialized = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;

        // strip quotes generated by serde_json
        let trimmed = serialized.trim_matches('"');

        write!(f, "{}", trimmed)
    }
}

// TODO(bitfl0wer): Test this!
impl TryFrom<&str> for DispatchEventType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // (ab)use serde_json to deserialize a Self variant from a SCREAMING_SNAKE_CASE string
        // we have to wrap the value in quotes to make it a valid JSON string. we also call .trim()
        // and .to_uppercase() to give a little more leeway to the caller
        serde_json::from_str::<Self>(&format!("\"{}\"", value.trim().to_uppercase()))
            .map_err(|e| Error::Custom(e.to_string()))
    }
}

impl TryFrom<String> for DispatchEventType {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        DispatchEventType::try_from(value.as_str())
    }
}

#[derive(Serialize, Clone, PartialEq, Debug)]
/// A de-/serializable data payload for transmission over the gateway.
pub struct GatewayPayload<T>
where
    T: Serialize + DeserializeOwned,
{
    #[serde(rename = "op")]
    pub op_code: u8,
    #[serde(rename = "d")]
    pub event_data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "s")]
    pub sequence_number: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "t")]
    pub event_name: Option<String>,
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
            None => return Err(::serde::de::Error::missing_field("d")),
        };
        let sequence_number = value.get("s").cloned().map(|v| v.as_u64().unwrap());
        let event_name = match value.get("t") {
            Some(v) => v.as_str().map(|v_str| v_str.to_string()),
            None => None,
        };
        Ok(GatewayPayload {
            op_code,
            event_data,
            sequence_number,
            event_name,
        })
    }
}

#[derive(Default, Clone)]
pub struct ConnectedUsers {
    pub store: Arc<RwLock<ConnectedUsersInner>>,
    pub role_user_map: Arc<Mutex<RoleUserMap>>,
}

/// A mapping of Snowflake IDs to the "inbox" of a [GatewayUser].
///
/// An "inbox" is a [tokio::sync::mpsc::Sender] that can be used to send [Event]s to all connected
/// clients of a [GatewayUser].
#[derive(Default)]
pub struct ConnectedUsersInner {
    pub inboxes: HashMap<Snowflake, tokio::sync::broadcast::Sender<Event>>,
    pub users: HashMap<Snowflake, Arc<Mutex<GatewayUser>>>,
    pub resumeable_clients_store: ResumableClientsStore,
}

/// A single identifiable User connected to the Gateway - possibly using many clients at the same
/// time.
pub struct GatewayUser {
    /// The "inbox" of a [GatewayUser]. This is a [tokio::sync::mpsc::Receiver]. Events sent to
    /// this inbox will be sent to all connected clients of this user.
    pub inbox: tokio::sync::broadcast::Receiver<Event>,
    /// The "outbox" of a [GatewayUser]. This is a [tokio::sync::mpsc::Sender]. From this outbox,
    /// more inboxes can be created.
    outbox: tokio::sync::broadcast::Sender<Event>,
    /// Sessions a User is connected with. HashMap of SessionToken -> GatewayClient
    clients: HashMap<String, Arc<Mutex<GatewayClient>>>,
    /// The Snowflake ID of the User.
    pub id: Snowflake,
    /// A collection of [Subscribers](Subscriber) to [Event] [Publishers](pubserve::Publisher).
    ///
    /// A GatewayUser may have many [GatewayClients](GatewayClient), but he only gets subscribed to
    /// all relevant [Publishers](pubserve::Publisher) *once* to save resources.
    subscriptions: Vec<Box<dyn Subscriber<Event>>>,
    /// [Weak] reference to the [ConnectedUsers] store.
    connected_users: ConnectedUsers,
}

/// A concrete session, that a [GatewayUser] is connected to the Gateway with.
pub struct GatewayClient {
    connection: WebSocketConnection,
    /// A [Weak] reference to the [GatewayUser] this client belongs to.
    pub parent: Weak<Mutex<GatewayUser>>,
    // Handle to the main Gateway task for this client
    main_task_handle: tokio::task::JoinHandle<()>,
    // Handle to the heartbeat task for this client
    heartbeat_task_handle: tokio::task::JoinHandle<()>,
    // Kill switch to disconnect the client
    pub kill_send: tokio::sync::broadcast::Sender<()>,
    /// Token of the session token used for this connection
    pub session_token: String,
    /// The last sequence number received from the client. Shared between the main task, heartbeat
    /// task, and this struct.
    last_sequence: Arc<Mutex<u64>>,
}

impl ConnectedUsers {
    /// Create a new, empty [ConnectedUsers] instance.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bulk_message_builder(&self) -> BulkMessageBuilder {
        BulkMessageBuilder::default()
    }

    /// Initialize the [RoleUserMap] with data from the database.
    ///
    /// This method will query the database for all roles and all users that have these roles.
    /// The data will then populate the map.
    ///
    /// Due to the possibly large number of roles and users returned by the database, this method
    /// should only be executed once. The [RoleUserMap] should be kept synchronized with the database
    /// through means that do not involve this method.
    ///
    /// ## Locking
    ///
    /// This method acquires a lock on `role_user_map` for the duration of its runtime.
    pub async fn init_role_user_map(&self, db: &PgPool) -> Result<(), crate::errors::Error> {
        self.role_user_map.lock().await.init(db).await
    }

    /// Get a [GatewayUser] by its Snowflake ID if it already exists in the store, or create a new
    /// [GatewayUser] if it does not exist using [ConnectedUsers::new_user].
    ///
    /// ## Locking
    ///
    /// This method always acquires a read lock on `store`. If the user does not yet exist in the
    /// store, a `write` lock will be acquired additionally.
    pub fn get_user_or_new(&self, id: Snowflake) -> Arc<Mutex<GatewayUser>> {
        let inner = self.store.clone();
        log::trace!(target: "symfonia::gateway::types::ConnectedUsers::get_user_or_new", "Acquiring lock on ConnectedUsersInner...");
        let mut lock = inner.read();
        log::trace!(target: "symfonia::gateway::types::ConnectedUsers::get_user_or_new", "Lock acquired!");
        if let Some(user) = lock.users.get(&id) {
            log::trace!(target: "symfonia::gateway::types::ConnectedUsers::get_user_or_new", "Found user {id} in store");
            user.clone()
        } else {
            drop(lock);
            log::trace!(target: "symfonia::gateway::types::ConnectedUsers::get_user_or_new", "Creating new user {id} in store");
            self.new_user(HashMap::new(), id, Vec::new())
        }
    }

    pub fn inner(&self) -> Arc<RwLock<ConnectedUsersInner>> {
        self.store.clone()
    }

    /// Register a new [GatewayUser] with the [ConnectedUsers] instance.
    ///
    /// ## Locking
    ///
    /// This method acquires a write lock on `store` for the duration of its runtime.
    fn register(&self, user: GatewayUser) -> Arc<Mutex<GatewayUser>> {
        log::trace!(target: "symfonia::gateway::types::ConnectedUsers::register", "Acquiring lock on ConnectedUsersInner...");
        self.store
            .write()
            .inboxes
            .insert(user.id, user.outbox.clone());
        log::trace!(target: "symfonia::gateway::types::ConnectedUsers::register", "Lock acquired!");
        let id = user.id;
        let arc = Arc::new(Mutex::new(user));
        self.store.write().users.insert(id, arc.clone());
        log::trace!(target: "symfonia::gateway::types::ConnectedUsers::register", "Inserted user {id} into users store");
        arc
    }

    /// Deregister a [GatewayUser] from the [ConnectedUsers] instance.
    ///
    /// ## Locking
    ///
    /// This method acquires a write lock on `store` for the duration of its runtime.
    pub fn deregister(&self, user: &GatewayUser) {
        self.store.write().inboxes.remove(&user.id);
        self.store.write().users.remove(&user.id);
    }

    /// Get the "inbox" of a [GatewayUser] by its Snowflake ID.
    ///
    /// ## Locking
    ///
    /// This method acquires a read lock on `store` for the duration of its runtime.
    pub async fn inbox(&self, id: Snowflake) -> Option<tokio::sync::broadcast::Sender<Event>> {
        self.store.read().inboxes.get(&id).cloned()
    }

    /// Create a new [GatewayUser] with the given Snowflake ID, [GatewayClient]s, and subscriptions.
    /// Registers the new [GatewayUser] with the [ConnectedUsers] instance.
    ///
    /// ## Locking
    ///
    /// This method calls [Self::register]. Refer to that method for information on locking behavior.
    pub fn new_user(
        &self,
        clients: HashMap<String, Arc<Mutex<GatewayClient>>>,
        id: Snowflake,
        subscriptions: Vec<Box<dyn Subscriber<Event>>>,
    ) -> Arc<Mutex<GatewayUser>> {
        let channel = tokio::sync::broadcast::channel(20);
        let user = GatewayUser {
            inbox: channel.1,
            outbox: channel.0.clone(),
            clients,
            id,
            subscriptions,
            connected_users: self.clone(),
        };
        self.register(user)
    }

    /// Create a new [GatewayClient] with the given [GatewayUser], [Connection], and other data.
    /// Also handles appending the new [GatewayClient] to the [GatewayUser]'s list of clients.
    ///
    /// ## Locking
    ///
    /// This method acquires a lock on the [Arc<Mutex<GatewayUser>>] that is passed as `user`.
    #[allow(clippy::too_many_arguments)]
    pub async fn new_client(
        &self,
        user: Arc<Mutex<GatewayUser>>,
        connection: WebSocketConnection,
        main_task_handle: tokio::task::JoinHandle<()>,
        heartbeat_task_handle: tokio::task::JoinHandle<()>,
        kill_send: tokio::sync::broadcast::Sender<()>,
        session_token: &str,
        last_sequence: Arc<Mutex<u64>>,
    ) -> Arc<Mutex<GatewayClient>> {
        let client = GatewayClient {
            connection,
            parent: Arc::downgrade(&user),
            main_task_handle,
            heartbeat_task_handle,
            kill_send,
            session_token: session_token.to_string(),
            last_sequence,
        };
        let arc = Arc::new(Mutex::new(client));
        log::trace!(target: "symfonia::gateway::ConnectedUsers::new_client", "Acquiring lock on user...");
        user.lock()
            .await
            .clients
            .insert(session_token.to_string(), arc.clone());
        log::trace!(target: "symfonia::gateway::ConnectedUsers::new_client", "Lock acquired!");
        log::trace!(target: "symfonia::gateway::ConnectedUsers::new_client", "Inserted into map. Done.");
        arc
    }
}

impl std::hash::Hash for GatewayUser {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for GatewayUser {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for GatewayUser {}

impl GatewayClient {
    /// Disconnects a [GatewayClient] properly, including un-registering it from the memory store
    /// and creating a resumeable session.
    pub async fn die(mut self, connected_users: ConnectedUsers) {
        self.kill_send.send(()).unwrap();
        let disconnect_info = DisconnectInfo {
            session_token: self.session_token.clone(),
            disconnected_at_sequence: *self.last_sequence.lock().await,
            parent: self.parent.clone(),
        };
        self.parent
            .upgrade()
            .unwrap()
            .lock()
            .await
            .clients
            .remove(&self.session_token);
        connected_users.deregister(self.parent.upgrade().unwrap().lock().await.deref());
        connected_users
            .store
            .write()
            .resumeable_clients_store
            .insert(self.session_token.clone(), disconnect_info);
    }
}

#[derive(Default, Clone)]
/// `BulkMessageBuilder` can be used to build and send GatewayMessages to the inboxes of all
/// currently connected [GatewayClients](GatewayClient). Recipients can be added either via
/// User or Role snowflake IDs.
pub struct BulkMessageBuilder {
    users: Vec<Snowflake>,
    roles: Vec<Snowflake>,
    message: Option<Event>,
}

impl BulkMessageBuilder {
    /// Add the given list of user snowflake IDs to the list of recipients.
    pub async fn add_user_recipients(&mut self, users: &[Snowflake]) {
        self.users.extend_from_slice(users);
    }

    /// Add all members which have the given role snowflake IDs to the list of recipients.
    pub async fn add_role_recipients(&mut self, roles: &[Snowflake]) {
        self.roles.extend_from_slice(roles);
    }

    /// Set the message to be sent to the recipients.
    pub async fn set_message(&mut self, message: Event) {
        self.message = Some(message);
    }

    /// Send the message to all recipients.
    pub async fn send(self, connected_users: ConnectedUsers) -> Result<(), crate::errors::Error> {
        if self.message.is_none() {
            return Err(crate::errors::Error::Custom(
                "No message to send".to_string(),
            ));
        }
        let mut recipients = HashSet::new();
        let lock = connected_users.role_user_map.lock().await;
        for role in self.roles.iter() {
            if let Some(users) = lock.get(role) {
                for user in users.iter() {
                    recipients.insert(*user);
                }
            }
            for user in self.users.iter() {
                recipients.insert(*user);
            }
        }
        if recipients.is_empty() {
            return Ok(());
        }
        for recipient in recipients.iter() {
            if let Some(inbox) = connected_users.inbox(*recipient).await {
                inbox.send(self.message.clone().unwrap()).map_err(|e| {
                    crate::errors::Error::Custom(format!("tokio broadcast error: {}", e))
                })?;
            }
        }
        Ok(())
    }
}

#[derive(Default)]
/// Represents all existing roles on the server and the users that have these roles.
pub struct RoleUserMap {
    /// Map Role Snowflake ID to a list of User Snowflake IDs
    map: HashMap<Snowflake, HashSet<Snowflake>>,
}

impl Deref for RoleUserMap {
    type Target = HashMap<Snowflake, HashSet<Snowflake>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for RoleUserMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl RoleUserMap {
    /// Initialize the [RoleUserMap] with data from the database.
    ///
    /// This method will query the database for all roles and all users that have these roles.
    /// The data will then populate the map.
    ///
    /// Due to the possibly large number of roles and users returned by the database, this method
    /// should only be executed once. The [RoleUserMap] should be kept synchronized with the database
    /// through means that do not involve this method.
    pub async fn init(&mut self, db: &PgPool) -> Result<(), crate::errors::Error> {
        // First, get all role ids from the roles table and insert them into the map
        let all_role_ids: Vec<PgU64> = sqlx::query_as("SELECT id FROM roles")
            .fetch_all(db)
            .await
            .map_err(crate::errors::Error::Sqlx)?;
        for role_id in all_role_ids.iter() {
            self.map
                .insert(Snowflake::from(role_id.to_uint()), HashSet::new());
        }
        // Then, query member_roles and insert the user ids into the map
        let all_member_roles: Vec<(PgU64, PgU64)> =
            sqlx::query_as("SELECT index, role_id FROM member_roles")
                .fetch_all(db)
                .await
                .map_err(crate::errors::Error::Sqlx)?;
        for (user_id, role_id) in all_member_roles.iter() {
            // Unwrapping is fine here, as the member_roles table has a foreign key constraint
            // which states that role_id must be a valid id in the roles table.
            let users_for_role_id = self.map.get_mut(&role_id.to_uint().into()).unwrap();
            users_for_role_id.insert(user_id.to_uint().into());
        }
        Ok(())
    }
}

/// Connection to a WebSocket client with sending and receiving capabilities.
///
/// A [WebSocketConnection] is essentially an adapter from tungstenites sink/stream to a
/// [tokio::sync::broadcast] channel. Broadcast channels are used in favor of sink/stream, because
/// to clone a sink/stream to pass it around to different tasks which need sending/receiving
/// capabilities, an `Arc<Mutex<T>>` has to be used. This means, that no more than one task can
/// listen for incoming messages at a time, as a lock on the [Mutex] has to be acquired.
///
/// Read up on [tokio::sync::broadcast] channels if you'd like to understand how they work.
pub struct WebSocketConnection {
    pub sender: tokio::sync::broadcast::Sender<Message>,
    pub receiver: tokio::sync::broadcast::Receiver<Message>,
    sender_task: Arc<tokio::task::JoinHandle<()>>,
    receiver_task: Arc<tokio::task::JoinHandle<()>>,
}

impl WebSocketConnection {
    /// Create a new [WebSocketConnection] from a tungstenite Sink/Stream pair.
    pub fn new(mut sink: WebSocketSend, mut stream: WebSocketReceive) -> Self {
        // "100" is an arbitrary limit. Feel free to adjust this, if you have a good reason for it. -bitfl0wer
        let (mut sender, mut receiver) = tokio::sync::broadcast::channel(100);
        let mut sender_sender_task = sender.clone();
        let mut receiver_sender_task = receiver.resubscribe();
        // The sender task concerns itself with sending messages to the WebSocket client.
        let sender_task = tokio::spawn(async move {
            log::trace!(target: "symfonia::gateway::types::WebSocketConnection", "spawned sender_task");
            loop {
                let message: Result<Message, tokio::sync::broadcast::error::RecvError> =
                    receiver_sender_task.recv().await;
                match message {
                    Ok(msg) => {
                        let send_result = sink.send(msg).await;
                        match send_result {
                            Ok(_) => (),
                            Err(_) => {
                                sender_sender_task.send(Message::Close(Some(CloseFrame {
                                    code: CloseCode::Error,
                                    reason: "Channel closed or error encountered".into(),
                                })));
                                return;
                            }
                        }
                    }
                    Err(_) => return,
                }
            }
        });
        let sender_receiver_task = sender.clone();
        // The receiver task receives messages from the WebSocket client and sends them to the
        // broadcast channel.
        let receiver_task = tokio::spawn(async move {
            log::trace!(target: "symfonia::gateway::types::WebSocketConnection", "spawned receiver_task");
            loop {
                let web_socket_receive_result = match stream.next().await {
                    Some(res) => res,
                    None => {
                        log::debug!(target: "symfonia::gateway::WebSocketConnection", "WebSocketReceive yielded None. Sending close message...");
                        sender_receiver_task.send(Message::Close(Some(CloseFrame {
                            code: CloseCode::Error,
                            reason: "Channel closed or error encountered".into(),
                        })));
                        return;
                    }
                };
                let web_socket_receive_message = match web_socket_receive_result {
                    Ok(message) => message,
                    Err(e) => {
                        log::error!(target: "symfonia::gateway::WebSocketConnection", "Received malformed message, closing channel: {e}");
                        sender_receiver_task.send(Message::Close(Some(CloseFrame {
                            code: CloseCode::Error,
                            reason: "Channel closed or error encountered".into(),
                        })));
                        return;
                    }
                };
                match sender_receiver_task.send(web_socket_receive_message) {
                    Ok(_) => (),
                    Err(e) => {
                        log::error!(target: "symfonia::gateway::WebSocketConnection", "Unable to send received WebSocket message to channel recipients. Closing channel: {e}");
                        sender_receiver_task.send(Message::Close(Some(CloseFrame {
                            code: CloseCode::Error,
                            reason: "Channel closed or error encountered".into(),
                        })));
                        return;
                    }
                }
            }
        });
        Self {
            sender,
            receiver,
            sender_task: Arc::new(sender_task),
            receiver_task: Arc::new(receiver_task),
        }
    }
}

impl Clone for WebSocketConnection {
    fn clone(&self) -> Self {
        log::trace!(target: "symfonia::gateway::WebSocketConnection", "WebSocketConnection cloned!");
        Self {
            sender: self.sender.clone(),
            receiver: self.receiver.resubscribe(),
            sender_task: self.sender_task.clone(),
            receiver_task: self.receiver_task.clone(),
        }
    }
}

#[derive(Clone)]
pub struct DisconnectInfo {
    /// session token that was used for this connection
    pub session_token: String,
    pub disconnected_at_sequence: u64,
    pub parent: Weak<Mutex<GatewayUser>>,
}

impl
    From<(
        SplitSink<WebSocketStream<TcpStream>, tokio_tungstenite::tungstenite::Message>,
        SplitStream<WebSocketStream<TcpStream>>,
    )> for WebSocketConnection
{
    fn from(
        value: (
            SplitSink<WebSocketStream<TcpStream>, tokio_tungstenite::tungstenite::Message>,
            SplitStream<WebSocketStream<TcpStream>>,
        ),
    ) -> Self {
        Self::new(value.0, value.1)
    }
}

/// Represents a new successful connection to the gateway. The user is already part of the [ConnectedUsers]
/// and the client is already registered with the [GatewayClient] "clients" map.
pub struct NewWebSocketConnection {
    pub user: Arc<Mutex<GatewayUser>>,
    pub client: Arc<Mutex<GatewayClient>>,
}

#[cfg(test)]
mod dispatch_event_type_tests {
    // One could stop and think: "Is this really necessary"? And my answer to that is: Would you
    // rather come to terms with one file being longer than you'd like, or would you rather have
    // weird deserialization errors and incompatibilities caused by a faulty Display implementation
    // which you have to debug months later?
    //
    // Thought so. -bitfl0wer
    //
    // P.S.: Testing all enum variants has resulted in me finding three bugs in the Display impl.
    use super::*;

    #[test]
    fn test_ready() {
        let event = DispatchEventType::Ready;
        assert_eq!(event.to_string(), "READY");
        assert_eq!(
            DispatchEventType::try_from("readY ".to_string()).unwrap(),
            event
        );
        assert_eq!(
            DispatchEventType::try_from("READY".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_ready_supplemental() {
        let event = DispatchEventType::ReadySupplemental;
        assert_eq!(event.to_string(), "READY_SUPPLEMENTAL");
        assert_eq!(
            DispatchEventType::try_from("READY_SUPPLEMENTAL".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_resumed() {
        let event = DispatchEventType::Resumed;
        assert_eq!(event.to_string(), "RESUMED");
        assert_eq!(
            DispatchEventType::try_from("RESUMED".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_auth_session_change() {
        let event = DispatchEventType::AuthSessionChange;
        assert_eq!(event.to_string(), "AUTH_SESSION_CHANGE");
        assert_eq!(
            DispatchEventType::try_from("AUTH_SESSION_CHANGE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_authenticator_create() {
        let event = DispatchEventType::AuthenticatorCreate;
        assert_eq!(event.to_string(), "AUTHENTICATOR_CREATE");
        assert_eq!(
            DispatchEventType::try_from("AUTHENTICATOR_CREATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_authenticator_update() {
        let event = DispatchEventType::AuthenticatorUpdate;
        assert_eq!(event.to_string(), "AUTHENTICATOR_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("AUTHENTICATOR_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_authenticator_delete() {
        let event = DispatchEventType::AuthenticatorDelete;
        assert_eq!(event.to_string(), "AUTHENTICATOR_DELETE");
        assert_eq!(
            DispatchEventType::try_from("AUTHENTICATOR_DELETE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_application_command_permissions_update() {
        let event = DispatchEventType::ApplicationCommandPermissionsUpdate;
        assert_eq!(event.to_string(), "APPLICATION_COMMAND_PERMISSIONS_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("APPLICATION_COMMAND_PERMISSIONS_UPDATE".to_string())
                .unwrap(),
            event
        );
    }

    #[test]
    fn test_auto_moderation_rule_create() {
        let event = DispatchEventType::AutoModerationRuleCreate;
        assert_eq!(event.to_string(), "AUTO_MODERATION_RULE_CREATE");
        assert_eq!(
            DispatchEventType::try_from("AUTO_MODERATION_RULE_CREATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_auto_moderation_rule_update() {
        let event = DispatchEventType::AutoModerationRuleUpdate;
        assert_eq!(event.to_string(), "AUTO_MODERATION_RULE_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("AUTO_MODERATION_RULE_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_auto_moderation_rule_delete() {
        let event = DispatchEventType::AutoModerationRuleDelete;
        assert_eq!(event.to_string(), "AUTO_MODERATION_RULE_DELETE");
        assert_eq!(
            DispatchEventType::try_from("AUTO_MODERATION_RULE_DELETE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_auto_moderation_action_execution() {
        let event = DispatchEventType::AutoModerationActionExecution;
        assert_eq!(event.to_string(), "AUTO_MODERATION_ACTION_EXECUTION");
        assert_eq!(
            DispatchEventType::try_from("AUTO_MODERATION_ACTION_EXECUTION".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_auto_moderation_mention_raid_detection() {
        let event = DispatchEventType::AutoModerationMentionRaidDetection;
        assert_eq!(event.to_string(), "AUTO_MODERATION_MENTION_RAID_DETECTION");
        assert_eq!(
            DispatchEventType::try_from("AUTO_MODERATION_MENTION_RAID_DETECTION".to_string())
                .unwrap(),
            event
        );
    }

    #[test]
    fn test_call_create() {
        let event = DispatchEventType::CallCreate;
        assert_eq!(event.to_string(), "CALL_CREATE");
        assert_eq!(
            DispatchEventType::try_from("CALL_CREATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_call_update() {
        let event = DispatchEventType::CallUpdate;
        assert_eq!(event.to_string(), "CALL_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("CALL_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_call_delete() {
        let event = DispatchEventType::CallDelete;
        assert_eq!(event.to_string(), "CALL_DELETE");
        assert_eq!(
            DispatchEventType::try_from("CALL_DELETE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_channel_create() {
        let event = DispatchEventType::ChannelCreate;
        assert_eq!(event.to_string(), "CHANNEL_CREATE");
        assert_eq!(
            DispatchEventType::try_from("CHANNEL_CREATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_channel_update() {
        let event = DispatchEventType::ChannelUpdate;
        assert_eq!(event.to_string(), "CHANNEL_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("CHANNEL_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_channel_delete() {
        let event = DispatchEventType::ChannelDelete;
        assert_eq!(event.to_string(), "CHANNEL_DELETE");
        assert_eq!(
            DispatchEventType::try_from("CHANNEL_DELETE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_channel_statuses() {
        let event = DispatchEventType::ChannelStatuses;
        assert_eq!(event.to_string(), "CHANNEL_STATUSES");
        assert_eq!(
            DispatchEventType::try_from("CHANNEL_STATUSES".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_voice_channel_status_update() {
        let event = DispatchEventType::VoiceChannelStatusUpdate;
        assert_eq!(event.to_string(), "VOICE_CHANNEL_STATUS_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("VOICE_CHANNEL_STATUS_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_channel_pins_update() {
        let event = DispatchEventType::ChannelPinsUpdate;
        assert_eq!(event.to_string(), "CHANNEL_PINS_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("CHANNEL_PINS_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_channel_recipient_add() {
        let event = DispatchEventType::ChannelRecipientAdd;
        assert_eq!(event.to_string(), "CHANNEL_RECIPIENT_ADD");
        assert_eq!(
            DispatchEventType::try_from("CHANNEL_RECIPIENT_ADD".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_channel_recipient_remove() {
        let event = DispatchEventType::ChannelRecipientRemove;
        assert_eq!(event.to_string(), "CHANNEL_RECIPIENT_REMOVE");
        assert_eq!(
            DispatchEventType::try_from("CHANNEL_RECIPIENT_REMOVE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_dm_settings_upsell_show() {
        let event = DispatchEventType::DmSettingsUpsellShow;
        assert_eq!(event.to_string(), "DM_SETTINGS_UPSELL_SHOW");
        assert_eq!(
            DispatchEventType::try_from("DM_SETTINGS_UPSELL_SHOW".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_thread_create() {
        let event = DispatchEventType::ThreadCreate;
        assert_eq!(event.to_string(), "THREAD_CREATE");
        assert_eq!(
            DispatchEventType::try_from("THREAD_CREATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_thread_update() {
        let event = DispatchEventType::ThreadUpdate;
        assert_eq!(event.to_string(), "THREAD_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("THREAD_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_thread_delete() {
        let event = DispatchEventType::ThreadDelete;
        assert_eq!(event.to_string(), "THREAD_DELETE");
        assert_eq!(
            DispatchEventType::try_from("THREAD_DELETE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_thread_list_sync() {
        let event = DispatchEventType::ThreadListSync;
        assert_eq!(event.to_string(), "THREAD_LIST_SYNC");
        assert_eq!(
            DispatchEventType::try_from("THREAD_LIST_SYNC".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_thread_member_update() {
        let event = DispatchEventType::ThreadMemberUpdate;
        assert_eq!(event.to_string(), "THREAD_MEMBER_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("THREAD_MEMBER_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_thread_members_update() {
        let event = DispatchEventType::ThreadMembersUpdate;
        assert_eq!(event.to_string(), "THREAD_MEMBERS_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("THREAD_MEMBERS_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_friend_suggestion_create() {
        let event = DispatchEventType::FriendSuggestionCreate;
        assert_eq!(event.to_string(), "FRIEND_SUGGESTION_CREATE");
        assert_eq!(
            DispatchEventType::try_from("FRIEND_SUGGESTION_CREATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_friend_suggestion_delete() {
        let event = DispatchEventType::FriendSuggestionDelete;
        assert_eq!(event.to_string(), "FRIEND_SUGGESTION_DELETE");
        assert_eq!(
            DispatchEventType::try_from("FRIEND_SUGGESTION_DELETE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_create() {
        let event = DispatchEventType::GuildCreate;
        assert_eq!(event.to_string(), "GUILD_CREATE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_CREATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_update() {
        let event = DispatchEventType::GuildUpdate;
        assert_eq!(event.to_string(), "GUILD_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_delete() {
        let event = DispatchEventType::GuildDelete;
        assert_eq!(event.to_string(), "GUILD_DELETE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_DELETE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_audit_log_entry_create() {
        let event = DispatchEventType::GuildAuditLogEntryCreate;
        assert_eq!(event.to_string(), "GUILD_AUDIT_LOG_ENTRY_CREATE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_AUDIT_LOG_ENTRY_CREATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_ban_add() {
        let event = DispatchEventType::GuildBanAdd;
        assert_eq!(event.to_string(), "GUILD_BAN_ADD");
        assert_eq!(
            DispatchEventType::try_from("GUILD_BAN_ADD".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_ban_remove() {
        let event = DispatchEventType::GuildBanRemove;
        assert_eq!(event.to_string(), "GUILD_BAN_REMOVE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_BAN_REMOVE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_emojis_update() {
        let event = DispatchEventType::GuildEmojisUpdate;
        assert_eq!(event.to_string(), "GUILD_EMOJIS_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_EMOJIS_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_stickers_update() {
        let event = DispatchEventType::GuildStickersUpdate;
        assert_eq!(event.to_string(), "GUILD_STICKERS_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_STICKERS_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_join_request_create() {
        let event = DispatchEventType::GuildJoinRequestCreate;
        assert_eq!(event.to_string(), "GUILD_JOIN_REQUEST_CREATE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_JOIN_REQUEST_CREATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_join_request_update() {
        let event = DispatchEventType::GuildJoinRequestUpdate;
        assert_eq!(event.to_string(), "GUILD_JOIN_REQUEST_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_JOIN_REQUEST_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_join_request_delete() {
        let event = DispatchEventType::GuildJoinRequestDelete;
        assert_eq!(event.to_string(), "GUILD_JOIN_REQUEST_DELETE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_JOIN_REQUEST_DELETE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_member_add() {
        let event = DispatchEventType::GuildMemberAdd;
        assert_eq!(event.to_string(), "GUILD_MEMBER_ADD");
        assert_eq!(
            DispatchEventType::try_from("GUILD_MEMBER_ADD".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_member_remove() {
        let event = DispatchEventType::GuildMemberRemove;
        assert_eq!(event.to_string(), "GUILD_MEMBER_REMOVE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_MEMBER_REMOVE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_member_update() {
        let event = DispatchEventType::GuildMemberUpdate;
        assert_eq!(event.to_string(), "GUILD_MEMBER_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_MEMBER_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_members_chunk() {
        let event = DispatchEventType::GuildMembersChunk;
        assert_eq!(event.to_string(), "GUILD_MEMBERS_CHUNK");
        assert_eq!(
            DispatchEventType::try_from("GUILD_MEMBERS_CHUNK".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_role_create() {
        let event = DispatchEventType::GuildRoleCreate;
        assert_eq!(event.to_string(), "GUILD_ROLE_CREATE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_ROLE_CREATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_role_update() {
        let event = DispatchEventType::GuildRoleUpdate;
        assert_eq!(event.to_string(), "GUILD_ROLE_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_ROLE_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_role_delete() {
        let event = DispatchEventType::GuildRoleDelete;
        assert_eq!(event.to_string(), "GUILD_ROLE_DELETE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_ROLE_DELETE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_scheduled_event_create() {
        let event = DispatchEventType::GuildScheduledEventCreate;
        assert_eq!(event.to_string(), "GUILD_SCHEDULED_EVENT_CREATE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_SCHEDULED_EVENT_CREATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_scheduled_event_update() {
        let event = DispatchEventType::GuildScheduledEventUpdate;
        assert_eq!(event.to_string(), "GUILD_SCHEDULED_EVENT_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_SCHEDULED_EVENT_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_scheduled_event_delete() {
        let event = DispatchEventType::GuildScheduledEventDelete;
        assert_eq!(event.to_string(), "GUILD_SCHEDULED_EVENT_DELETE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_SCHEDULED_EVENT_DELETE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_scheduled_event_user_add() {
        let event = DispatchEventType::GuildScheduledEventUserAdd;
        assert_eq!(event.to_string(), "GUILD_SCHEDULED_EVENT_USER_ADD");
        assert_eq!(
            DispatchEventType::try_from("GUILD_SCHEDULED_EVENT_USER_ADD".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_scheduled_event_user_remove() {
        let event = DispatchEventType::GuildScheduledEventUserRemove;
        assert_eq!(event.to_string(), "GUILD_SCHEDULED_EVENT_USER_REMOVE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_SCHEDULED_EVENT_USER_REMOVE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_soundboard_sound_create() {
        let event = DispatchEventType::GuildSoundboardSoundCreate;
        assert_eq!(event.to_string(), "GUILD_SOUNDBOARD_SOUND_CREATE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_SOUNDBOARD_SOUND_CREATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_soundboard_sound_update() {
        let event = DispatchEventType::GuildSoundboardSoundUpdate;
        assert_eq!(event.to_string(), "GUILD_SOUNDBOARD_SOUND_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_SOUNDBOARD_SOUND_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_soundboard_sound_delete() {
        let event = DispatchEventType::GuildSoundboardSoundDelete;
        assert_eq!(event.to_string(), "GUILD_SOUNDBOARD_SOUND_DELETE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_SOUNDBOARD_SOUND_DELETE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_soundboard_sounds() {
        let event = DispatchEventType::SoundboardSounds;
        assert_eq!(event.to_string(), "SOUNDBOARD_SOUNDS");
        assert_eq!(
            DispatchEventType::try_from("SOUNDBOARD_SOUNDS".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_guild_integrations_update() {
        let event = DispatchEventType::GuildIntegrationsUpdate;
        assert_eq!(event.to_string(), "GUILD_INTEGRATIONS_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("GUILD_INTEGRATIONS_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_integration_create() {
        let event = DispatchEventType::IntegrationCreate;
        assert_eq!(event.to_string(), "INTEGRATION_CREATE");
        assert_eq!(
            DispatchEventType::try_from("INTEGRATION_CREATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_integration_update() {
        let event = DispatchEventType::IntegrationUpdate;
        assert_eq!(event.to_string(), "INTEGRATION_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("INTEGRATION_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_integration_delete() {
        let event = DispatchEventType::IntegrationDelete;
        assert_eq!(event.to_string(), "INTEGRATION_DELETE");
        assert_eq!(
            DispatchEventType::try_from("INTEGRATION_DELETE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_interaction_create() {
        let event = DispatchEventType::InteractionCreate;
        assert_eq!(event.to_string(), "INTERACTION_CREATE");
        assert_eq!(
            DispatchEventType::try_from("INTERACTION_CREATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_invite_create() {
        let event = DispatchEventType::InviteCreate;
        assert_eq!(event.to_string(), "INVITE_CREATE");
        assert_eq!(
            DispatchEventType::try_from("INVITE_CREATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_invite_delete() {
        let event = DispatchEventType::InviteDelete;
        assert_eq!(event.to_string(), "INVITE_DELETE");
        assert_eq!(
            DispatchEventType::try_from("INVITE_DELETE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_message_create() {
        let event = DispatchEventType::MessageCreate;
        assert_eq!(event.to_string(), "MESSAGE_CREATE");
        assert_eq!(
            DispatchEventType::try_from("MESSAGE_CREATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_message_update() {
        let event = DispatchEventType::MessageUpdate;
        assert_eq!(event.to_string(), "MESSAGE_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("MESSAGE_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_message_delete() {
        let event = DispatchEventType::MessageDelete;
        assert_eq!(event.to_string(), "MESSAGE_DELETE");
        assert_eq!(
            DispatchEventType::try_from("MESSAGE_DELETE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_message_delete_bulk() {
        let event = DispatchEventType::MessageDeleteBulk;
        assert_eq!(event.to_string(), "MESSAGE_DELETE_BULK");
        assert_eq!(
            DispatchEventType::try_from("MESSAGE_DELETE_BULK".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_message_reaction_add() {
        let event = DispatchEventType::MessageReactionAdd;
        assert_eq!(event.to_string(), "MESSAGE_REACTION_ADD");
        assert_eq!(
            DispatchEventType::try_from("MESSAGE_REACTION_ADD".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_message_reaction_remove() {
        let event = DispatchEventType::MessageReactionRemove;
        assert_eq!(event.to_string(), "MESSAGE_REACTION_REMOVE");
        assert_eq!(
            DispatchEventType::try_from("MESSAGE_REACTION_REMOVE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_message_reaction_remove_all() {
        let event = DispatchEventType::MessageReactionRemoveAll;
        assert_eq!(event.to_string(), "MESSAGE_REACTION_REMOVE_ALL");
        assert_eq!(
            DispatchEventType::try_from("MESSAGE_REACTION_REMOVE_ALL".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_message_reaction_remove_emoji() {
        let event = DispatchEventType::MessageReactionRemoveEmoji;
        assert_eq!(event.to_string(), "MESSAGE_REACTION_REMOVE_EMOJI");
        assert_eq!(
            DispatchEventType::try_from("MESSAGE_REACTION_REMOVE_EMOJI".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_presence_update() {
        let event = DispatchEventType::PresenceUpdate;
        assert_eq!(event.to_string(), "PRESENCE_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("PRESENCE_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_typing_start() {
        let event = DispatchEventType::TypingStart;
        assert_eq!(event.to_string(), "TYPING_START");
        assert_eq!(
            DispatchEventType::try_from("TYPING_START".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_user_update() {
        let event = DispatchEventType::UserUpdate;
        assert_eq!(event.to_string(), "USER_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("USER_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_voice_state_update() {
        let event = DispatchEventType::VoiceStateUpdate;
        assert_eq!(event.to_string(), "VOICE_STATE_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("VOICE_STATE_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_voice_server_update() {
        let event = DispatchEventType::VoiceServerUpdate;
        assert_eq!(event.to_string(), "VOICE_SERVER_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("VOICE_SERVER_UPDATE".to_string()).unwrap(),
            event
        );
    }

    #[test]
    fn test_webhooks_update() {
        let event = DispatchEventType::WebhooksUpdate;
        assert_eq!(event.to_string(), "WEBHOOKS_UPDATE");
        assert_eq!(
            DispatchEventType::try_from("WEBHOOKS_UPDATE".to_string()).unwrap(),
            event
        );
    }
}
