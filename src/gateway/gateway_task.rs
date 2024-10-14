use std::{sync::Arc, time::Duration};

use chorus::types::{GatewayHeartbeat, GatewaySendPayload};
use futures::StreamExt;
use serde_json::from_str;
use tokio::{sync::Mutex, time::sleep};
use tokio_tungstenite::tungstenite::{protocol::CloseFrame, Message};

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
                    Ok(message) => {
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
