use std::sync::Arc;

use chorus::types::GatewayHeartbeat;
use futures::StreamExt;
use tokio::sync::Mutex;

use crate::errors::Error;

use super::{Event, GatewayClient};

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
