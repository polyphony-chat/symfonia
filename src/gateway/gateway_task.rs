use std::sync::Arc;

use tokio::sync::Mutex;

use crate::errors::Error;

use super::{Event, GatewayClient};

/// Handles all messages a client sends to the gateway post-handshake.
pub(super) async fn gateway_task(
    connection: Arc<Mutex<super::WebSocketConnection>>,
    inbox: tokio::sync::broadcast::Receiver<Event>,
    kill_receive: tokio::sync::broadcast::Receiver<()>,
    kill_send: tokio::sync::broadcast::Sender<()>,
    last_sequence_number: Arc<Mutex<u64>>,
) {
    // TODO
    todo!()
}

async fn handle_event(event: Event, connection: Arc<Mutex<super::WebSocketConnection>>) {
    // TODO
    todo!()
}

async fn process_inbox(
    mut inbox: tokio::sync::broadcast::Receiver<Event>,
    mut kill_receive: tokio::sync::broadcast::Receiver<()>,
) -> Result<(), Error> {
    tokio::select! {
        _ = kill_receive.recv() => {
            Ok(())
        }
        event = inbox.recv() => {
            match event {
                Ok(event) => {
                    // TODO
                    todo!()
                }
                Err(_) => {
                    Ok(())
                }
            }
        }
    }
}
