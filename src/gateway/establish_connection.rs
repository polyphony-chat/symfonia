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

    let (kill_send, mut kill_receive) = tokio::sync::broadcast::channel(1);
    let (message_send, message_receive) = tokio::sync::broadcast::channel(4);
    let sequence_number = Arc::new(Mutex::new(0u64));
    let (session_id_send, session_id_receive) = tokio::sync::broadcast::channel(1);

    // This JoinHandle `.is_some()` if we receive a heartbeat message *before* we receive an
    // identify or resume message.
    let mut heartbeat_handler_handle: Option<JoinHandle<()>> = None;

    tokio::select! {
        _ = kill_receive.recv() => {
            return Err(GatewayError::Closed.into());
        }
        _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
            return Err(GatewayError::Timeout.into());
        }
        else => {
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
                    match heartbeat_handler_handle {
                        None => {
                            // This only happens *once*. You will find that we have to `.resubscribe()` to
                            // the channels to make the borrow checker happy, because the channels are otherwise
                            // moved into the spawned task, which, *technically* could occur multiple times,
                            // due to the loop {} construct. However, this is not the case, because this code
                            // executes only if heartbeat_handler_handle is None, which is only true once,
                            // as we set it to Some(_) in this block. We could perhaps make this a little
                            // nicer by using unsafe rust magic, which would also allow us to use more appropriate
                            // channel types such as `oneshot` for the session_id_receive channel. However,
                            // I don't see that this is needed at the moment.
                            heartbeat_handler_handle = Some(tokio::spawn({
                                let mut heartbeat_handler = HeartbeatHandler::new(
                                    connection.clone(),
                                    kill_receive.resubscribe(),
                                    kill_send.clone(),
                                    message_receive.resubscribe(),
                                    sequence_number.clone(),
                                    session_id_receive.resubscribe(),
                                );
                                async move {
                                    heartbeat_handler.run().await;
                                }
                            }))
                        }
                        Some(_) => {
                            message_send.send(heartbeat);
                        }
                    }
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
        }
    }

    todo!()
}
