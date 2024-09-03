// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use ::serde::de::DeserializeOwned;
use ::serde::{Deserialize, Serialize};
use chorus::types::*;

#[derive(
    Debug,
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Copy,
    Hash,
)]
/// Enum representing all possible* event types that can be received from or sent to the gateway.
///
/// TODO: This is only temporary. Replace with this enum from chorus, when it is ready.
pub enum EventType {
    Hello,
    Ready,
    Resumed,
    InvalidSession,
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    ChannelPinsUpdate,
    ThreadCreate,
    ThreadUpdate,
    ThreadDelete,
    ThreadListSync,
    ThreadMemberUpdate,
    ThreadMembersUpdate,
    GuildCreate,
    GuildUpdate,
    GuildDelete,
    GuildBanAdd,
    GuildBanRemove,
    GuildEmojisUpdate,
    GuildIntegrationsUpdate,
    GuildMemberAdd,
    GuildMemberRemove,
    GuildMemberUpdate,
    GuildMembersChunk,
    GuildRoleCreate,
    GuildRoleUpdate,
    GuildRoleDelete,
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
    MessageReactionAdd,
    MessageReactionRemove,
    MessageReactionRemoveAll,
    MessageReactionRemoveEmoji,
    PresenceUpdate,
    TypingStart,
    UserUpdate,
    VoiceStateUpdate,
    VoiceServerUpdate,
    WebhooksUpdate,
    StageInstanceCreate,
    StageInstanceUpdate,
    StageInstanceDelete,
    RequestMembers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Enum representing all possible* events that can be received from or sent to the gateway.
///
/// TODO: This is only temporary. Replace with this enum from chorus, when it is ready.
#[serde(rename_all = "PascalCase")]
pub enum Event {
    Hello(GatewayHello),
    Ready(GatewayReady),
    Resumed(GatewayResume),
    InvalidSession(GatewayInvalidSession),
    ChannelCreate(ChannelCreate),
    ChannelUpdate(ChannelUpdate),
    ChannelDelete(ChannelDelete),
    ThreadCreate(ThreadCreate),
    ThreadUpdate(ThreadUpdate),
    ThreadDelete(ThreadDelete),
    ThreadListSync(ThreadListSync),
    ThreadMemberUpdate(ThreadMemberUpdate),
    ThreadMembersUpdate(ThreadMembersUpdate),
    GuildCreate(GuildCreate),
    GuildUpdate(GuildUpdate),
    GuildDelete(GuildDelete),
    GuildBanAdd(GuildBanAdd),
    GuildBanRemove(GuildBanRemove),
    GuildEmojisUpdate(GuildEmojisUpdate),
    GuildIntegrationsUpdate(GuildIntegrationsUpdate),
    GuildMemberAdd(GuildMemberAdd),
    GuildMemberRemove(GuildMemberRemove),
    GuildMemberUpdate(GuildMemberUpdate),
    GuildMembersChunk(GuildMembersChunk),
    InteractionCreate(InteractionCreate),
    InviteCreate(InviteCreate),
    InviteDelete(InviteDelete),
    MessageCreate(MessageCreate),
    MessageUpdate(MessageUpdate),
    MessageDelete(MessageDelete),
    MessageDeleteBulk(MessageDeleteBulk),
    MessageReactionAdd(MessageReactionAdd),
    MessageReactionRemove(MessageReactionRemove),
    MessageReactionRemoveAll(MessageReactionRemoveAll),
    MessageReactionRemoveEmoji(MessageReactionRemoveEmoji),
    PresenceUpdate(PresenceUpdate),
    TypingStart(TypingStartEvent),
    UserUpdate(UserUpdate),
    VoiceStateUpdate(VoiceStateUpdate),
    VoiceServerUpdate(VoiceServerUpdate),
    WebhooksUpdate(WebhooksUpdate),
    StageInstanceCreate(StageInstanceCreate),
    StageInstanceUpdate(StageInstanceUpdate),
    StageInstanceDelete(StageInstanceDelete),
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
