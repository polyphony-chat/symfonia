use std::{sync::Arc, time::Duration};

use chorus::types::{GatewayHeartbeat, GatewaySendPayload, Opcode};
use futures::StreamExt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use tokio::{sync::Mutex, time::sleep};
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::{protocol::CloseFrame, Message};

use crate::errors::{Error, GatewayError};
use crate::gateway::{DispatchEvent, DispatchEventType};

use super::{Event, GatewayClient, GatewayPayload};

/// Handles all messages a client sends to the gateway post-handshake.
pub(super) async fn gateway_task(
    mut connection: super::WebSocketConnection,
    mut inbox: tokio::sync::broadcast::Receiver<Event>,
    mut heartbeat_send: tokio::sync::broadcast::Sender<GatewayHeartbeat>,
    last_sequence_number: Arc<Mutex<u64>>,
) {
    log::trace!(target: "symfonia::gateway::gateway_task", "Started a new gateway task!");
    let inbox_processor = tokio::spawn(process_inbox(connection.clone(), inbox.resubscribe()));

    /*
    Before we can respond to any gateway event we receive, we need to figure out what kind of event
    we are dealing with. For a lot of events, this is easy, because we can just look at the opcode
    and figure out the event type. For the dispatch events however, we also need to look at the event
    name to find out the exact dispatch event we are dealing with. -bitfl0wer
     */

    loop {
        tokio::select! {
            _ = connection.kill_receive.recv() => {
                return;
            },
            message_result = connection.receiver.recv() => {
                match message_result {
                    Ok(message_of_unknown_type) => {
                        let event = unwrap_event(Event::try_from(message_of_unknown_type), connection.clone(), connection.kill_send.clone());
                        // TODO: Handle event
                    },
                    Err(error) => {
                        connection.sender.send(Message::Close(Some(CloseFrame { code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Library(4000), reason: "INTERNAL_SERVER_ERROR".into() })));
                        connection.kill_send.send(()).expect("Failed to send kill_send");
                        return;
                    },
                }
            }
        }
    }

    todo!()
}

fn handle_event(
    event: Event,
    connection: super::WebSocketConnection,
    mut kill_send: tokio::sync::broadcast::Sender<()>,
) {
    todo!()
}

/// Unwraps an event from a Result<Event, Error> and handles the error if there is one. Errors will
/// shut down all tasks belonging to this session and will kill the gateway task through a panic.
fn unwrap_event(
    result: Result<Event, Error>,
    connection: super::WebSocketConnection,
    mut kill_send: tokio::sync::broadcast::Sender<()>,
) -> Event {
    match result {
        Err(e) => {
            match e {
                Error::Gateway(g) => match g {
                    GatewayError::UnexpectedOpcode(o) => {
                        log::debug!(target: "symfonia::gateway::gateway_task", "Received an unexpected opcode: {:?}", o);
                        connection.sender.send(Message::Close(Some(CloseFrame { code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Library(4001), reason: "UNKNOWN_OPCODE".into() })));
                        kill_send.send(()).expect("Failed to send kill_send");
                        panic!("Killing gateway task: Received an unexpected opcode");
                    }
                    GatewayError::UnexpectedMessage(m) => {
                        log::debug!(target: "symfonia::gateway::gateway_task", "Received an unexpected message: {:?}", m);
                        connection.sender.send(Message::Close(Some(CloseFrame { code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Library(4002), reason: "DECODE_ERROR".into() })));
                        kill_send.send(()).expect("Failed to send kill_send");
                        panic!("Killing gateway task: Received an unexpected message");
                    }
                    _ => {
                        log::debug!(target: "symfonia::gateway::gateway_task", "Received an unexpected error: {:?}", g);
                        connection.sender.send(Message::Close(Some(CloseFrame { code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Library(4000), reason: "INTERNAL_SERVER_ERROR".into() })));
                        kill_send.send(()).expect("Failed to send kill_send");
                        panic!("Killing gateway task: Received an unexpected error");
                    }
                },
                _ => {
                    log::debug!(target: "symfonia::gateway::gateway_task", "Received an unexpected error: {:?}", e);
                    connection.sender.send(Message::Close(Some(CloseFrame { code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Library(4000), reason: "INTERNAL_SERVER_ERROR".into() })));
                    kill_send.send(()).expect("Failed to send kill_send");
                    panic!("Killing gateway task: Received an unexpected error");
                }
            }
        }
        Ok(event) => event,
    }
}

async fn process_inbox(
    mut connection: super::WebSocketConnection,
    mut inbox: tokio::sync::broadcast::Receiver<Event>,
) {
    loop {
        tokio::select! {
            _ = connection.kill_receive.recv() => {
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
