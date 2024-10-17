use std::{sync::Arc, time::Duration};

use chorus::types::{GatewayHeartbeat, GatewaySendPayload, Opcode};
use futures::StreamExt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use tokio::{sync::Mutex, time::sleep};
use tokio_tungstenite::tungstenite::{protocol::CloseFrame, Message};

use crate::errors::{Error, GatewayError};
use crate::gateway::DispatchEventType;

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
