use std::{sync::Arc, time::Duration};

use chorus::types::{GatewayHeartbeat, GatewaySendPayload, Opcode};
use futures::StreamExt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use tokio::{sync::Mutex, time::sleep};
use tokio_tungstenite::tungstenite::{protocol::CloseFrame, Message};

use crate::errors::{Error, GatewayError};
use crate::gateway::{DispatchEvent, DispatchEventType};

use super::{Event, GatewayClient, GatewayPayload};

/// Handles all messages a client sends to the gateway post-handshake.
pub(super) async fn gateway_task(
    mut connection: super::WebSocketConnection,
    mut inbox: tokio::sync::broadcast::Receiver<Event>,
    mut kill_receive: tokio::sync::broadcast::Receiver<()>,
    mut kill_send: tokio::sync::broadcast::Sender<()>,
    mut heartbeat_send: tokio::sync::broadcast::Sender<GatewayHeartbeat>,
    last_sequence_number: Arc<Mutex<u64>>,
) {
    log::trace!(target: "symfonia::gateway::gateway_task", "Started a new gateway task!");
    let inbox_processor = tokio::spawn(process_inbox(
        connection.clone(),
        inbox.resubscribe(),
        kill_receive.resubscribe(),
    ));

    /*
    Before we can respond to any gateway event we receive, we need to figure out what kind of event
    we are dealing with. For a lot of events, this is easy, because we can just look at the opcode
    and figure out the event type. For the dispatch events however, we also need to look at the event
    name to find out the exact dispatch event we are dealing with. -bitfl0wer
     */

    loop {
        tokio::select! {
            _ = kill_receive.recv() => {
                return;
            },
            message_result = connection.receiver.recv() => {
                match message_result {
                    Ok(message_of_unknown_type) => {
                        todo!()
                        // TODO: Do something with the event
                    },
                    Err(error) => {
                        connection.sender.send(Message::Close(Some(CloseFrame { code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Library(4000), reason: "INTERNAL_SERVER_ERROR".into() })));
                        kill_send.send(()).expect("Failed to send kill_send");
                        return;
                    },
                }
            }
        }
    }

    todo!()
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

/// Takes a message of unknown type as input and tries to convert it to an [Event].
fn match_message_type(message: Message) -> Result<Event, Error> {
    let message_as_string = message.to_string();
    let raw_gateway_payload: GatewayPayload<String> = from_str(&message_as_string)?;
    match Opcode::try_from(raw_gateway_payload.op_code).map_err(|_| {
        Error::Gateway(GatewayError::UnexpectedMessage(format!(
            "Unknown Opcode: {}",
            raw_gateway_payload.op_code
        )))
    })? {
        Opcode::Heartbeat => return convert_to!(Event::Heartbeat, message_as_string),
        Opcode::Identify => return convert_to!(Event::Heartbeat, message_as_string),
        Opcode::PresenceUpdate => return convert_to!(Event::PresenceUpdate, message_as_string),
        Opcode::VoiceStateUpdate => return convert_to!(Event::VoiceStateUpdate, message_as_string),
        Opcode::VoiceServerPing => return convert_to!(Event::VoiceServerPing, message_as_string),
        Opcode::Resume => return convert_to!(Event::Resume, message_as_string),
        Opcode::Reconnect => return convert_to!(Event::Reconnect, message_as_string),
        Opcode::RequestGuildMembers => {
            return convert_to!(Event::RequestGuildMembers, message_as_string)
        }
        Opcode::InvalidSession => return convert_to!(Event::InvalidSession, message_as_string),
        Opcode::Hello => return convert_to!(Event::Hello, message_as_string),
        Opcode::HeartbeatAck => return convert_to!(Event::HeartbeatAck, message_as_string),
        #[allow(deprecated)]
        Opcode::GuildSync => {
            return Err(Error::Gateway(GatewayError::UnexpectedMessage(format!(
                "Deprecated opcode: {}",
                raw_gateway_payload.op_code
            ))))
        }
        Opcode::CallConnect => return convert_to!(Event::CallConnect, message_as_string),
        Opcode::GuildSubscriptions => {
            return convert_to!(Event::GuildSubscriptions, message_as_string)
        }
        Opcode::LobbyConnect => return convert_to!(Event::LobbyConnect, message_as_string),
        Opcode::LobbyDisconnect => return convert_to!(Event::LobbyDisconnect, message_as_string),
        Opcode::LobbyVoiceStates => return convert_to!(Event::LobbyVoiceStates, message_as_string),
        Opcode::StreamCreate => return convert_to!(Event::StreamCreate, message_as_string),
        Opcode::StreamDelete => return convert_to!(Event::StreamDelete, message_as_string),
        Opcode::StreamWatch => return convert_to!(Event::StreamWatch, message_as_string),
        Opcode::StreamPing => return convert_to!(Event::StreamPing, message_as_string),
        Opcode::StreamSetPaused => return convert_to!(Event::StreamSetPaused, message_as_string),
        #[allow(deprecated)]
        Opcode::LfgSubscriptions => {
            return Err(Error::Gateway(GatewayError::UnexpectedMessage(format!(
                "Deprecated opcode {} will not be processed",
                raw_gateway_payload.op_code
            ))))
        }
        #[allow(deprecated)]
        Opcode::RequestGuildApplicationCommands => {
            return Err(Error::Gateway(GatewayError::UnexpectedMessage(format!(
                "Deprecated opcode {} will not be processed",
                raw_gateway_payload.op_code
            ))))
        }
        Opcode::EmbeddedActivityCreate => {
            return convert_to!(Event::EmbeddedActivityCreate, message_as_string)
        }
        Opcode::EmbeddedActivityDelete => {
            return convert_to!(Event::EmbeddedActivityDelete, message_as_string)
        }
        Opcode::EmbeddedActivityUpdate => {
            return convert_to!(Event::EmbeddedActivityUpdate, message_as_string)
        }
        Opcode::RequestForumUnreads => {
            return convert_to!(Event::RequestForumUnreads, message_as_string)
        }
        Opcode::RemoteCommand => return convert_to!(Event::RemoteCommand, message_as_string),
        Opcode::RequestDeletedEntityIDs => {
            return convert_to!(Event::RequestDeletedEntityIDs, message_as_string)
        }
        Opcode::RequestSoundboardSounds => {
            return convert_to!(Event::RequestSoundboardSounds, message_as_string)
        }
        Opcode::SpeedTestCreate => return convert_to!(Event::SpeedTestCreate, message_as_string),
        Opcode::SpeedTestDelete => return convert_to!(Event::SpeedTestDelete, message_as_string),
        Opcode::RequestLastMessages => {
            return convert_to!(Event::RequestLastMessages, message_as_string)
        }
        Opcode::SearchRecentMembers => {
            return convert_to!(Event::SearchRecentMembers, message_as_string)
        }
        Opcode::RequestChannelStatuses => {
            return convert_to!(Event::RequestChannelStatuses, message_as_string)
        }
        // Dispatch has to be handled differently. To not nest further, we just do nothing here,
        // then handle it outside of this
        Opcode::Dispatch => (),
        o => {
            return Err(GatewayError::UnexpectedMessage(format!(
                "Opcode not implemented: {}",
                o as u8
            ))
            .into())
        }
    };

    let dispatch_event_name = match raw_gateway_payload.event_name {
        Some(n) => n,
        None => {
            return Err(GatewayError::UnexpectedMessage(format!(
                "No event name provided on dispatch event: {}",
                message_as_string
            ))
            .into())
        }
    };

    let dispatch_event_type =
        DispatchEventType::try_from(dispatch_event_name.as_str()).map_err(|_| {
            GatewayError::UnexpectedMessage(format!(
                "Unknown dispatch event: {}",
                dispatch_event_name
            ))
        })?;

    // At this point we know what Dispatch event we are dealing with and can try to deserialize it

    // TODO: Turn this into a declarative macro if possible to reduce code duplication
    match dispatch_event_type {
        DispatchEventType::Ready => {
            return convert_to!(DispatchEvent::Ready, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::ReadySupplemental => {
            return convert_to!(DispatchEvent::ReadySupplemental, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::Resumed => {
            return convert_to!(DispatchEvent::Resumed, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::AuthSessionChange => {
            return convert_to!(DispatchEvent::AuthSessionChange, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::AuthenticatorCreate => {
            return convert_to!(DispatchEvent::AuthenticatorCreate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::AuthenticatorUpdate => {
            return convert_to!(DispatchEvent::AuthenticatorUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::AuthenticatorDelete => {
            return convert_to!(DispatchEvent::AuthenticatorDelete, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::ApplicationCommandPermissionsUpdate => {
            return convert_to!(
                DispatchEvent::ApplicationCommandPermissionsUpdate,
                message_as_string
            )
            .map(Event::Dispatch)
        }
        DispatchEventType::AutoModerationRuleCreate => {
            return convert_to!(DispatchEvent::AutoModerationRuleCreate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::AutoModerationRuleUpdate => {
            return convert_to!(DispatchEvent::AutoModerationRuleUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::AutoModerationRuleDelete => {
            return convert_to!(DispatchEvent::AutoModerationRuleDelete, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::AutoModerationActionExecution => {
            return convert_to!(
                DispatchEvent::AutoModerationActionExecution,
                message_as_string
            )
            .map(Event::Dispatch)
        }
        DispatchEventType::AutoModerationMentionRaidDetection => {
            return convert_to!(
                DispatchEvent::AutoModerationMentionRaidDetection,
                message_as_string
            )
            .map(Event::Dispatch)
        }
        DispatchEventType::CallCreate => {
            return convert_to!(DispatchEvent::CallCreate, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::CallUpdate => {
            return convert_to!(DispatchEvent::CallUpdate, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::CallDelete => {
            return convert_to!(DispatchEvent::CallDelete, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::ChannelCreate => {
            return convert_to!(DispatchEvent::ChannelCreate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::ChannelUpdate => {
            return convert_to!(DispatchEvent::ChannelUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::ChannelDelete => {
            return convert_to!(DispatchEvent::ChannelDelete, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::ChannelStatuses => {
            return convert_to!(DispatchEvent::ChannelStatuses, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::VoiceChannelStatusUpdate => {
            return convert_to!(DispatchEvent::VoiceChannelStatusUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::ChannelPinsUpdate => {
            return convert_to!(DispatchEvent::ChannelPinsUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::ChannelRecipientAdd => {
            return convert_to!(DispatchEvent::ChannelRecipientAdd, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::ChannelRecipientRemove => {
            return convert_to!(DispatchEvent::ChannelRecipientRemove, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::DmSettingsUpsellShow => {
            return convert_to!(DispatchEvent::DmSettingsUpsellShow, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::ThreadCreate => {
            return convert_to!(DispatchEvent::ThreadCreate, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::ThreadUpdate => {
            return convert_to!(DispatchEvent::ThreadUpdate, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::ThreadDelete => {
            return convert_to!(DispatchEvent::ThreadDelete, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::ThreadListSync => {
            return convert_to!(DispatchEvent::ThreadListSync, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::ThreadMemberUpdate => {
            return convert_to!(DispatchEvent::ThreadMemberUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::ThreadMembersUpdate => {
            return convert_to!(DispatchEvent::ThreadMembersUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::FriendSuggestionCreate => {
            return convert_to!(DispatchEvent::FriendSuggestionCreate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::FriendSuggestionDelete => {
            return convert_to!(DispatchEvent::FriendSuggestionDelete, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildCreate => {
            return convert_to!(DispatchEvent::GuildCreate, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::GuildUpdate => {
            return convert_to!(DispatchEvent::GuildUpdate, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::GuildDelete => {
            return convert_to!(DispatchEvent::GuildDelete, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::GuildAuditLogEntryCreate => {
            return convert_to!(DispatchEvent::GuildAuditLogEntryCreate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildBanAdd => {
            return convert_to!(DispatchEvent::GuildBanAdd, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::GuildBanRemove => {
            return convert_to!(DispatchEvent::GuildBanRemove, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildEmojisUpdate => {
            return convert_to!(DispatchEvent::GuildEmojisUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildStickersUpdate => {
            return convert_to!(DispatchEvent::GuildStickersUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildJoinRequestCreate => {
            return convert_to!(DispatchEvent::GuildJoinRequestCreate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildJoinRequestUpdate => {
            return convert_to!(DispatchEvent::GuildJoinRequestUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildJoinRequestDelete => {
            return convert_to!(DispatchEvent::GuildJoinRequestDelete, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildMemberAdd => {
            return convert_to!(DispatchEvent::GuildMemberAdd, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildMemberRemove => {
            return convert_to!(DispatchEvent::GuildMemberRemove, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildMemberUpdate => {
            return convert_to!(DispatchEvent::GuildMemberUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildMembersChunk => {
            return convert_to!(DispatchEvent::GuildMembersChunk, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildRoleCreate => {
            return convert_to!(DispatchEvent::GuildRoleCreate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildRoleUpdate => {
            return convert_to!(DispatchEvent::GuildRoleUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildRoleDelete => {
            return convert_to!(DispatchEvent::GuildRoleDelete, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildScheduledEventCreate => {
            return convert_to!(DispatchEvent::GuildScheduledEventCreate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildScheduledEventUpdate => {
            return convert_to!(DispatchEvent::GuildScheduledEventUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildScheduledEventDelete => {
            return convert_to!(DispatchEvent::GuildScheduledEventDelete, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildScheduledEventUserAdd => {
            return convert_to!(DispatchEvent::GuildScheduledEventUserAdd, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildScheduledEventUserRemove => {
            return convert_to!(
                DispatchEvent::GuildScheduledEventUserRemove,
                message_as_string
            )
            .map(Event::Dispatch)
        }
        DispatchEventType::GuildSoundboardSoundCreate => {
            return convert_to!(DispatchEvent::GuildSoundboardSoundCreate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildSoundboardSoundUpdate => {
            return convert_to!(DispatchEvent::GuildSoundboardSoundUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildSoundboardSoundDelete => {
            return convert_to!(DispatchEvent::GuildSoundboardSoundDelete, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::SoundboardSounds => {
            return convert_to!(DispatchEvent::SoundboardSounds, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::GuildIntegrationsUpdate => {
            return convert_to!(DispatchEvent::GuildIntegrationsUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::IntegrationCreate => {
            return convert_to!(DispatchEvent::IntegrationCreate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::IntegrationUpdate => {
            return convert_to!(DispatchEvent::IntegrationUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::IntegrationDelete => {
            return convert_to!(DispatchEvent::IntegrationDelete, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::InteractionCreate => {
            return convert_to!(DispatchEvent::InteractionCreate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::InviteCreate => {
            return convert_to!(DispatchEvent::InviteCreate, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::InviteDelete => {
            return convert_to!(DispatchEvent::InviteDelete, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::MessageCreate => {
            return convert_to!(DispatchEvent::MessageCreate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::MessageUpdate => {
            return convert_to!(DispatchEvent::MessageUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::MessageDelete => {
            return convert_to!(DispatchEvent::MessageDelete, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::MessageDeleteBulk => {
            return convert_to!(DispatchEvent::MessageDeleteBulk, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::MessagePollVoteAdd => {
            return convert_to!(DispatchEvent::MessagePollVoteAdd, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::MessagePollVoteRemove => {
            return convert_to!(DispatchEvent::MessagePollVoteRemove, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::MessageReactionAdd => {
            return convert_to!(DispatchEvent::MessageReactionAdd, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::MessageReactionAddMany => {
            return convert_to!(DispatchEvent::MessageReactionAddMany, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::MessageReactionRemove => {
            return convert_to!(DispatchEvent::MessageReactionRemove, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::MessageReactionRemoveAll => {
            return convert_to!(DispatchEvent::MessageReactionRemoveAll, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::MessageReactionRemoveEmoji => {
            return convert_to!(DispatchEvent::MessageReactionRemoveEmoji, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::RecentMentionDelete => {
            return convert_to!(DispatchEvent::RecentMentionDelete, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::LastMessages => {
            return convert_to!(DispatchEvent::LastMessages, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::Oauth2TokenRevoke => {
            return convert_to!(DispatchEvent::Oauth2TokenRevoke, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::PresenceUpdate => {
            return convert_to!(DispatchEvent::PresenceUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::RelationshipAdd => {
            return convert_to!(DispatchEvent::RelationshipAdd, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::RelationshipUpdate => {
            return convert_to!(DispatchEvent::RelationshipUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::RelationshipRemove => {
            return convert_to!(DispatchEvent::RelationshipRemove, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::StageInstanceCreate => {
            return convert_to!(DispatchEvent::StageInstanceCreate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::StageInstanceUpdate => {
            return convert_to!(DispatchEvent::StageInstanceUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::StageInstanceDelete => {
            return convert_to!(DispatchEvent::StageInstanceDelete, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::TypingStart => {
            return convert_to!(DispatchEvent::TypingStart, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::UserUpdate => {
            return convert_to!(DispatchEvent::UserUpdate, message_as_string).map(Event::Dispatch)
        }
        DispatchEventType::UserApplicationRemove => {
            return convert_to!(DispatchEvent::UserApplicationRemove, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::UserConnectionsUpdate => {
            return convert_to!(DispatchEvent::UserConnectionsUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::UserNoteUpdate => {
            return convert_to!(DispatchEvent::UserNoteUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::UserRequiredActionUpdate => {
            return convert_to!(DispatchEvent::UserRequiredActionUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::UserSettingsUpdate => {
            return convert_to!(DispatchEvent::UserSettingsUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::VoiceStateUpdate => {
            return convert_to!(DispatchEvent::VoiceStateUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::VoiceServerUpdate => {
            return convert_to!(DispatchEvent::VoiceServerUpdate, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::VoiceChannelEffectSend => {
            return convert_to!(DispatchEvent::VoiceChannelEffectSend, message_as_string)
                .map(Event::Dispatch)
        }
        DispatchEventType::WebhooksUpdate => {
            return convert_to!(DispatchEvent::WebhooksUpdate, message_as_string)
                .map(Event::Dispatch)
        }
    }

    todo!()
}

async fn process_inbox(
    connection: super::WebSocketConnection,
    mut inbox: tokio::sync::broadcast::Receiver<Event>,
    mut kill_receive: tokio::sync::broadcast::Receiver<()>,
) {
    loop {
        tokio::select! {
            _ = kill_receive.recv() => {
                return;
            }
            event = inbox.recv() => {
                match event {
                    Ok(event) => {
                        todo!();
                        // TODO: Process event
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
        }
    }
}
