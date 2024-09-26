use std::sync::Arc;

use chorus::types::{GatewayHeartbeat, GatewaySendPayload};
use futures::StreamExt;
use serde_json::from_str;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

use crate::errors::Error;

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
    let inbox_processor = tokio::spawn(process_inbox(
        connection.clone(),
        inbox.resubscribe(),
        kill_receive.resubscribe(),
    ));

    loop {
        todo!();
        tokio::select! {
            _ = kill_receive.recv() => {
                return;
            },
            // TODO: This locks the connection mutex which is not ideal/deadlock risk. Perhaps we
            // should turn our websocket connection into a tokio broadcast channel instead. so that
            // we can receive messages from it without having to lock one connection object.
        }
    }

    todo!()
}

/// Convert a [Message] into an [Event], if the event message is a valid event that a server can
/// expect to receive from a client.
fn received_message_to_event(message: Message) -> Result<Event, Error> {
    if !message.is_text() {
        return Err(Error::Custom(
            "Tungstenite message must be of type text".to_string(),
        ));
    }
    let message_text = message.to_string();
    let gateway_payload = from_str::<GatewayPayload<String>>(&message_text)?;
    match gateway_payload.op_code {
        1 => Ok(Event::Heartbeat(from_str(&message_text)?)),
        2 => Ok(Event::Identify(from_str(&message_text)?)),
        3 => Ok(Event::PresenceUpdate(from_str(&message_text)?)),
        4 => Ok(Event::VoiceStateUpdate(from_str(&message_text)?)),
        6 => Ok(Event::Resume(from_str(&message_text)?)),
        8 => Ok(Event::GuildMembersRequest(from_str(&message_text)?)),
        o => Err(Error::Custom(format!(
            "opcode {o} is not a valid event to receive from a client"
        ))),
    }
}

async fn handle_event(event: Event, connection: super::WebSocketConnection) -> Result<(), Error> {
    // TODO
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
                        handle_event(event, connection.clone()).await;
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
        }
    }
}
