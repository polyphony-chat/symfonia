use std::{collections::HashMap, sync::Arc};

use chorus::types::{
    GatewayHeartbeat, GatewayHeartbeatAck, GatewayHello, GatewayIdentifyPayload, GatewayResume,
    Snowflake,
};
use futures::{SinkExt, StreamExt};
use log::{debug, trace};
use rand::seq;
use serde_json::{from_str, json};
use sqlx::PgPool;
use tokio::{
    net::TcpStream,
    sync::{broadcast::Sender, Mutex},
    task::JoinHandle,
};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{
        protocol::{frame::coding::CloseCode, CloseFrame},
        Message,
    },
};

use crate::{
    database::entities::Config,
    errors::{Error, GatewayError},
    gateway::{gateway_task, heartbeat::HeartbeatHandler, GatewayPayload, GatewayUser},
    util::token::check_token,
};

use super::{ConnectedUsers, Connection, GatewayClient, NewConnection, ResumableClientsStore};

/// Internal use only state struct to pass around data to the `finish_connecting` function.
struct State {
    connection: Arc<Mutex<Connection>>,
    db: PgPool,
    config: Config,
    connected_users: ConnectedUsers,
    sequence_number: Arc<Mutex<u64>>,
    kill_send: Sender<()>,
    kill_receive: tokio::sync::broadcast::Receiver<()>,
    /// Receiver for heartbeat messages. The `HeartbeatHandler` will receive messages from this channel.
    heartbeat_receive: tokio::sync::broadcast::Receiver<GatewayHeartbeat>,
    /// Sender for heartbeat messages. The main gateway task will send messages to this channel for the `HeartbeatHandler` to receive and handle.
    heartbeat_send: tokio::sync::broadcast::Sender<GatewayHeartbeat>,
    session_id_receive: tokio::sync::broadcast::Receiver<String>,
}

/// `establish_connection` is the entrypoint method that gets called when a client tries to connect
/// to the WebSocket server.
///
/// If successful, returns a [NewConnection] with a new [Arc<Mutex<GatewayUser>>] and a
/// [GatewayClient], whose `.parent` field contains a [Weak] reference to the new [GatewayUser].
pub(super) async fn establish_connection(
    stream: TcpStream,
    db: PgPool,
    config: Config,
    connected_users: ConnectedUsers,
) -> Result<NewConnection, Error> {
    trace!(target: "symfonia::gateway::establish_connection::establish_connection", "Beginning process to establish connection (handshake)");
    // Accept the connection and split it into its sender and receiver halves.
    let ws_stream = accept_async(stream).await?;
    let mut connection: Connection = ws_stream.split().into();
    trace!(target: "symfonia::gateway::establish_connection::establish_connection", "Sending hello message");
    // Hello message
    connection
        .sender
        .send(Message::Text(json!(GatewayHello::default()).to_string()))
        .await?;
    trace!(target: "symfonia::gateway::establish_connection::establish_connection", "Sent hello message");

    // Wrap the connection in an Arc<Mutex<Connection>> to allow for shared ownership and mutation.
    // For example, the `HeartbeatHandler` and `GatewayClient` tasks will need to access the connection
    // to send messages.
    let connection = Arc::new(Mutex::new(connection));

    let mut received_identify_or_resume = false;

    let (kill_send, mut kill_receive) = tokio::sync::broadcast::channel::<()>(1);
    // Inter-task communication channels. The main gateway task will send received heartbeat related messages
    // to the `HeartbeatHandler` task via the `message_send` channel, which the `HeartbeatHandler` task will
    // then receive and handle.
    //
    // TODO: The HeartbeatHandler theoretically does not need a full connection object, but either only the sender
    // or just these message_* channels. Using the latter approach, the HeartbeatHandler could send Heartbeat
    // responses to the main gateway task, which in turn would send them to the client. This way, the
    // connection object might not need to be wraped in an `Arc<Mutex<Connection>>`.
    let (message_send, message_receive) = tokio::sync::broadcast::channel::<GatewayHeartbeat>(4);

    let sequence_number = Arc::new(Mutex::new(0u64)); // TODO: Actually use this, as in: Increment it when needed. Currently, this is not being done.

    // Used to inform the `HeartbeatHandler` task of the session_id of the client, if we receive it after a heartbeat handler task has been spawned.
    let (session_id_send, session_id_receive) = tokio::sync::broadcast::channel::<String>(1);

    let state = State {
        connection: connection.clone(),
        db: db.clone(),
        config: config.clone(),
        connected_users: connected_users.clone(),
        sequence_number: sequence_number.clone(),
        kill_send: kill_send.clone(),
        kill_receive: kill_receive.resubscribe(),
        heartbeat_receive: message_receive.resubscribe(),
        heartbeat_send: message_send.clone(),
        session_id_receive: session_id_receive.resubscribe(),
    };

    // This JoinHandle `.is_some()` if we receive a heartbeat message *before* we receive an
    // identify or resume message.
    let mut heartbeat_handler_handle: Option<JoinHandle<()>> = None;

    trace!(target: "symfonia::gateway::establish_connection::establish_connection", "Waiting for next message, timeout or kill signal...");
    let mut second_kill_receive = kill_receive.resubscribe();
    // Either we time out, the connection is killed, or we receive succesful output from `finish_connecting`.
    tokio::select! {
        _ = second_kill_receive.recv() => {
            debug!(target: "symfonia::gateway::establish_connection::establish_connection", "Connection was closed before we could establish it");
            Err(GatewayError::Closed.into())
        }
        // If we do not receive an identifying or resuming message within 30 seconds, we close the connection.
        _ = tokio::time::sleep(std::time::Duration::from_secs(30)) => {
            debug!(target: "symfonia::gateway::establish_connection::establish_connection", "Connection timed out: No message received within 30 seconds");
            Err(GatewayError::Timeout.into())
        }
        // Since async closures are not yet stable, we have to use a dedicated function to handle the
        // connection establishment process. :(
        new_connection = finish_connecting(heartbeat_handler_handle, state)
         => {
            new_connection
        }
    }
}

/// `finish_connecting` is the second part of the connection establishment process. It picks up after
/// the initial `Hello` message has been sent to the client. It then waits on the next message from
/// the client, which should be either a `Heartbeat`, `Identify` or `Resume` message, handling each
/// case accordingly.
#[allow(clippy::too_many_arguments)]
async fn finish_connecting(
    mut heartbeat_handler_handle: Option<JoinHandle<()>>,
    state: State,
) -> Result<NewConnection, Error> {
    loop {
        trace!(target: "symfonia::gateway::establish_connection::finish_connecting", "Waiting for next message...");
        let raw_message = match state.connection.lock().await.receiver.next().await {
            Some(next) => next,
            None => return Err(GatewayError::Timeout.into()),
        }?;
        debug!(target: "symfonia::gateway::establish_connection::finish_connecting", "Received message");

        if let Ok(heartbeat) = from_str::<GatewayHeartbeat>(&raw_message.to_string()) {
            log::trace!(target: "symfonia::gateway::establish_connection::finish_connecting", "Received heartbeat");
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
                            state.connection.clone(),
                            state.kill_receive.resubscribe(),
                            state.kill_send.clone(),
                            state.heartbeat_receive.resubscribe(),
                            state.sequence_number.clone(),
                            state.session_id_receive.resubscribe(),
                        );
                        async move {
                            heartbeat_handler.run().await;
                        }
                    }))
                }
                Some(_) => {
                    state.heartbeat_send.send(heartbeat);
                }
            }
        } else if let Ok(identify) =
            from_str::<GatewayPayload<GatewayIdentifyPayload>>(&raw_message.to_string())
        {
            log::trace!(target: "symfonia::gateway::establish_connection::finish_connecting", "Received identify payload");
            let claims = match check_token(
                &state.db,
                &identify.event_data.token,
                &state.config.security.jwt_secret,
            )
            .await
            {
                Ok(claims) => {
                    trace!(target: "symfonia::gateway::establish_connection::finish_connecting", "Token verified");
                    claims
                }
                Err(_) => {
                    log::trace!(target: "symfonia::gateway::establish_connection::finish_connecting", "Failed to verify token");
                    state
                        .kill_send
                        .send(())
                        .expect("Failed to send kill signal");
                    return Err(crate::errors::UserError::InvalidToken.into());
                }
            };
            let mut gateway_user = state.connected_users.get_user_or_new(claims.id).await;
            let gateway_client = state
                .connected_users
                .new_client(
                    gateway_user.clone(),
                    state.connection.clone(),
                    tokio::spawn(gateway_task::gateway_task(state.connection.clone())),
                    match heartbeat_handler_handle {
                        Some(handle) => handle,
                        None => tokio::spawn({
                            let mut heartbeat_handler = HeartbeatHandler::new(
                                state.connection.clone(),
                                state.kill_receive.resubscribe(),
                                state.kill_send.clone(),
                                state.heartbeat_receive.resubscribe(),
                                state.sequence_number.clone(),
                                state.session_id_receive.resubscribe(),
                            );
                            async move {
                                heartbeat_handler.run().await;
                            }
                        }),
                    },
                    state.kill_send.clone(),
                    &identify.event_data.token,
                    state.sequence_number.clone(),
                )
                .await;

            return Ok(NewConnection {
                user: gateway_user,
                client: gateway_client.clone(),
            });
        } else if let Ok(resume) = from_str::<GatewayResume>(&raw_message.to_string()) {
            log::trace!(target: "symfonia::gateway::establish_connection::finish_connecting", "Received resume payload");
            log::warn!(target: "symfonia::gateway::establish_connection::finish_connecting", "Resuming connections is not yet implemented. Telling client to identify instead.");
            state
                .connection
                .lock()
                .await
                .sender
                .send(Message::Close(Some(CloseFrame {
                    code: CloseCode::from(4007u16),
                    reason: "Resuming connections is not yet implemented. Please identify instead."
                        .into(),
                })))
                .await?;
            state
                .kill_send
                .send(())
                .expect("Failed to send kill signal");
        } else {
            debug!(target: "symfonia::gateway::establish_connection::finish_connecting", "Message could not be decoded as resume, heartbeat or identify.");

            return Err(GatewayError::UnexpectedMessage.into());
        }
    }
}
