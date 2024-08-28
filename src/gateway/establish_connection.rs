use std::sync::Arc;

use chorus::types::{
    GatewayHeartbeat, GatewayHeartbeatAck, GatewayHello, GatewayIdentifyPayload, GatewayResume,
    Snowflake,
};
use futures::{SinkExt, StreamExt};
use serde_json::{from_str, json};
use sqlx::PgPool;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

use crate::database::entities::Config;
use crate::errors::{Error, GatewayError};
use crate::gateway::heartbeat::HeartbeatHandler;
use crate::gateway::resume_connection::resume_connection;
use crate::gateway::GatewayUser;

use super::{Connection, GatewayClient, NewConnection};

/// `establish_connection` is the entrypoint method that gets called when a client tries to connect
/// to the WebSocket server.
///
/// If successful, returns a [NewConnection] with a new [Arc<Mutex<GatewayUser>>] and a
/// [GatewayClient], whose `.parent` field contains a [Weak] reference to the new [GatewayUser].
pub(super) async fn establish_connection(
    stream: TcpStream,
    db: PgPool, // TODO: Do we need db here?
    config: Config,
) -> Result<NewConnection, Error> {
    let ws_stream = accept_async(stream).await?;
    let mut connection: Connection = ws_stream.split().into();
    // Hello message
    connection
        .sender
        .send(Message::Text(json!(GatewayHello::default()).to_string()))
        .await?;

    let connection = Arc::new(Mutex::new(connection));

    let mut received_identify_or_resume = false;

    let (kill_send, kill_receive) = tokio::sync::broadcast::channel(1);
    let (message_send, message_receive) = tokio::sync::mpsc::channel(4);
    let sequence_number = Arc::new(Mutex::new(0u64));
    let mut heartbeat_handler = HeartbeatHandler::new(
        connection.clone(),
        kill_receive.resubscribe(),
        kill_send.clone(),
        message_receive,
        sequence_number.clone(),
    );

    // This JoinHandle `.is_some()` if we receive a heartbeat message *before* we receive an
    // identify or resume message.
    let heartbeat_handler_handle: Option<JoinHandle<()>>;

    loop {
        if received_identify_or_resume {
            break;
        }

        let raw_message = match connection.lock().await.receiver.next().await {
            Some(next) => next,
            None => return Err(GatewayError::Timeout.into()),
        }?;

        if let Ok(heartbeat) = from_str::<GatewayHeartbeat>(&raw_message.to_string()) {
            log::trace!(target: "symfonia::gateway::establish_connection", "Received heartbeat");
            heartbeat_handler_handle = Some(tokio::spawn(heartbeat_handler.run()));
        } else if let Ok(identify) = from_str::<GatewayIdentifyPayload>(&raw_message.to_string()) {
            received_identify_or_resume = true;
            log::trace!(target: "symfonia::gateway::establish_connection", "Received identify payload");
            // TODO: Verify token, build NewConnection
        } else if let Ok(resume) = from_str::<GatewayResume>(&raw_message.to_string()) {
            received_identify_or_resume = true;
            log::trace!(target: "symfonia::gateway::establish_connection", "Received resume payload");
            return resume_connection(connection, db, config, resume).await;
        } else {
            return Err(GatewayError::UnexpectedMessage.into());
        }
    }

    todo!()
}
