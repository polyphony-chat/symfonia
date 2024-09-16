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

use super::{Connection, GatewayClient, GatewayUsersStore, NewConnection, ResumableClientsStore};

/// `establish_connection` is the entrypoint method that gets called when a client tries to connect
/// to the WebSocket server.
///
/// If successful, returns a [NewConnection] with a new [Arc<Mutex<GatewayUser>>] and a
/// [GatewayClient], whose `.parent` field contains a [Weak] reference to the new [GatewayUser].
pub(super) async fn establish_connection(
    stream: TcpStream,
    db: PgPool,
    config: Config,
    gateway_users_store: GatewayUsersStore,
    resumeable_clients_store: ResumableClientsStore,
) -> Result<NewConnection, Error> {
    trace!(target: "symfonia::gateway::establish_connection::establish_connection", "Beginning process to establish connection (handshake)");
    let ws_stream = accept_async(stream).await?;
    let mut connection: Connection = ws_stream.split().into();
    trace!(target: "symfonia::gateway::establish_connection::establish_connection", "Sending hello message");
    // Hello message
    connection
        .sender
        .send(Message::Text(json!(GatewayHello::default()).to_string()))
        .await?;
    trace!(target: "symfonia::gateway::establish_connection::establish_connection", "Sent hello message");

    let connection = Arc::new(Mutex::new(connection));

    let mut received_identify_or_resume = false;

    let (kill_send, mut kill_receive) = tokio::sync::broadcast::channel::<()>(1);
    let (message_send, message_receive) = tokio::sync::broadcast::channel::<GatewayHeartbeat>(4);
    let sequence_number = Arc::new(Mutex::new(0u64)); // TODO: Actually use this, as in: Increment it when needed. Currently, this is not being done.
    let (session_id_send, session_id_receive) = tokio::sync::broadcast::channel::<String>(1);

    // This JoinHandle `.is_some()` if we receive a heartbeat message *before* we receive an
    // identify or resume message.
    let mut heartbeat_handler_handle: Option<JoinHandle<()>> = None;

    trace!(target: "symfonia::gateway::establish_connection::establish_connection", "Waiting for next message, timeout or kill signal...");
    let mut second_kill_receive = kill_receive.resubscribe();
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
        new_connection = finish_connecting(
            connection.clone(),
            heartbeat_handler_handle,
            kill_receive,
            kill_send,
            message_receive,
            message_send,
            sequence_number,
            session_id_receive,
            db,
            &config,
            gateway_users_store.clone(),
            resumeable_clients_store.clone(),
        ) => {
            new_connection
        }
    }
}

/// `get_or_new_gateway_user` is a helper function that retrieves a [GatewayUser] from the store if it exists,
/// or creates a new user, stores it in the store and then returns it, if it does not exist.
// TODO: Refactor this function according to the new `ResumeableClientsStore` definition.
async fn get_or_new_gateway_user(
    user_id: Snowflake,
    store: GatewayUsersStore,
    resumeable_clients_store: ResumableClientsStore,
) -> Arc<tokio::sync::Mutex<GatewayUser>> {
    let mut store = store.lock().await;
    if let Some(user) = store.get(&user_id) {
        return user.clone();
    }
    let user = Arc::new(Mutex::new(GatewayUser {
        id: user_id,
        clients: HashMap::new(),
        subscriptions: Vec::new(),
        resumeable_clients_store: Arc::downgrade(&resumeable_clients_store),
    }));
    store.insert(user_id, user.clone());
    user
}

/// `finish_connecting` is the second part of the connection establishment process. It picks up after
/// the initial `Hello` message has been sent to the client. It then waits on the next message from
/// the client, which should be either a `Heartbeat`, `Identify` or `Resume` message, handling each
/// case accordingly.
#[allow(clippy::too_many_arguments)]
async fn finish_connecting(
    connection: Arc<Mutex<Connection>>,
    mut heartbeat_handler_handle: Option<JoinHandle<()>>,
    kill_receive: tokio::sync::broadcast::Receiver<()>,
    kill_send: tokio::sync::broadcast::Sender<()>,
    message_receive: tokio::sync::broadcast::Receiver<GatewayHeartbeat>,
    message_send: tokio::sync::broadcast::Sender<GatewayHeartbeat>,
    sequence_number: Arc<Mutex<u64>>,
    session_id_receive: tokio::sync::broadcast::Receiver<String>,
    db: PgPool,
    config: &Config,
    gateway_users_store: GatewayUsersStore,
    resumeable_clients_store: ResumableClientsStore,
) -> Result<NewConnection, Error> {
    loop {
        trace!(target: "symfonia::gateway::establish_connection::finish_connecting", "Waiting for next message...");
        let raw_message = match connection.lock().await.receiver.next().await {
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
        } else if let Ok(identify) =
            from_str::<GatewayPayload<GatewayIdentifyPayload>>(&raw_message.to_string())
        {
            log::trace!(target: "symfonia::gateway::establish_connection::finish_connecting", "Received identify payload");
            let claims = match check_token(
                &db,
                &identify.event_data.token,
                &config.security.jwt_secret,
            )
            .await
            {
                Ok(claims) => {
                    trace!(target: "symfonia::gateway::establish_connection::finish_connecting", "Token verified");
                    claims
                }
                Err(_) => {
                    log::trace!(target: "symfonia::gateway::establish_connection::finish_connecting", "Failed to verify token");
                    kill_send.send(()).expect("Failed to send kill signal");
                    return Err(crate::errors::UserError::InvalidToken.into());
                }
            };
            let mut gateway_user = get_or_new_gateway_user(
                claims.id,
                gateway_users_store.clone(),
                resumeable_clients_store.clone(),
            )
            .await;
            let gateway_client = GatewayClient {
                parent: Arc::downgrade(&gateway_user),
                connection: connection.clone(),
                main_task_handle: tokio::spawn(gateway_task::gateway_task(connection.clone())),
                heartbeat_task_handle: match heartbeat_handler_handle {
                    Some(handle) => handle,
                    None => tokio::spawn({
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
                    }),
                },
                kill_send,
                session_token: identify.event_data.token,
                last_sequence: sequence_number.clone(),
            };
            let gateway_client_arc_mutex = Arc::new(Mutex::new(gateway_client));
            gateway_user.lock().await.clients.insert(
                gateway_client_arc_mutex.lock().await.session_token.clone(),
                gateway_client_arc_mutex.clone(),
            );
            return Ok(NewConnection {
                user: gateway_user,
                client: gateway_client_arc_mutex.clone(),
            });
        } else if let Ok(resume) = from_str::<GatewayResume>(&raw_message.to_string()) {
            log::trace!(target: "symfonia::gateway::establish_connection::finish_connecting", "Received resume payload");
            log::warn!(target: "symfonia::gateway::establish_connection::finish_connecting", "Resuming connections is not yet implemented. Telling client to identify instead.");
            connection
                .lock()
                .await
                .sender
                .send(Message::Close(Some(CloseFrame {
                    code: CloseCode::from(4000u16),
                    reason: "Resuming connections is not yet implemented. Please identify instead."
                        .into(),
                })))
                .await?;
            kill_send.send(()).expect("Failed to send kill signal");
        } else {
            debug!(target: "symfonia::gateway::establish_connection::finish_connecting", "Message could not be decoded as resume, heartbeat or identify.");

            return Err(GatewayError::UnexpectedMessage.into());
        }
    }
}
