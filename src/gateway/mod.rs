/*
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

static RESUME_RECONNECT_WINDOW_SECONDS: u8 = 90;

mod establish_connection;
mod gateway_task;
mod heartbeat;
mod resume_connection;
mod types;

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, Weak};
use std::thread::sleep;
use std::time::Duration;

use chorus::types::{
    GatewayHeartbeat, GatewayHello, GatewayIdentifyPayload, GatewayResume, Snowflake,
};
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use log::info;
use pubserve::Subscriber;
use serde_json::{from_str, json};
use sqlx::PgPool;
use tokio::net::{TcpListener, TcpStream};

use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{accept_async, WebSocketStream};
pub use types::*;

use crate::database::entities::Config;
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::errors::{Error, GatewayError};
use crate::util::token::check_token;
use crate::{SharedEventPublisherMap, WebSocketReceive, WebSocketSend};

/* NOTES (bitfl0wer) [These will be removed]
The gateway is supposed to be highly concurrent. It will be handling a lot of connections at once.
Thus, it makes sense to have each user connection be handled in a separate task.

It is important to make a distinction between the user and the client. A user can potentially
be connected with many devices at once. They are still just one user. Respecting this fact
will likely save a lot of computational power.

Handling a connection involves the following steps:

1. Accepting the connection
2. Sending a hello event back
3. Receiving a Heartbeat event
4. Returning a Heartbeat ACK event
5. Receiving an Identify payload <- "GatewayUser" and/or "GatewayClient" are instantiated here.
6. Responding with a Ready event

Handling disconnects and session resumes is for later and not considered at this exact moment.

From there on, run a task that takes ownership of the GatewayClient struct. This task will be what
is sending the events that the (to be implemented) Subscribers receive from the Publishers that the
GatewayUser is subscribed to
*/

/// A single identifiable User connected to the Gateway - possibly using many clients at the same
/// time.
struct GatewayUser {
    /// Sessions a User is connected with.
    clients: Vec<Arc<Mutex<GatewayClient>>>,
    /// The Snowflake ID of the User.
    id: Snowflake,
    /// A collection of [Subscribers](Subscriber) to [Event] [Publishers](pubserve::Publisher).
    ///
    /// A GatewayUser may have many [GatewayClients](GatewayClient), but he only gets subscribed to
    /// all relevant [Publishers](pubserve::Publisher) *once* to save resources.
    subscriptions: Vec<Box<dyn Subscriber<Event>>>,
}

/// A concrete session, that a [GatewayUser] is connected to the Gateway with.
struct GatewayClient {
    connection: Arc<Mutex<Connection>>,
    /// A [Weak] reference to the [GatewayUser] this client belongs to.
    parent: Weak<Mutex<GatewayUser>>,
    // Handle to the main Gateway task for this client
    main_task_handle: tokio::task::JoinHandle<Result<(), Error>>,
    // Handle to the heartbeat task for this client
    heartbeat_task_handle: tokio::task::JoinHandle<Result<(), Error>>,
    // Kill switch to disconnect the client
    kill_send: tokio::sync::broadcast::Sender<()>,
}

struct Connection {
    sender: WebSocketSend,
    receiver: WebSocketReceive,
}

struct DisconnectInfo {
    session_id: String,
    disconnected_at: u64,
    with_opcode: u16,
}

impl
    From<(
        SplitSink<WebSocketStream<TcpStream>, Message>,
        SplitStream<WebSocketStream<TcpStream>>,
    )> for Connection
{
    fn from(
        value: (
            SplitSink<WebSocketStream<TcpStream>, Message>,
            SplitStream<WebSocketStream<TcpStream>>,
        ),
    ) -> Self {
        Self {
            sender: value.0,
            receiver: value.1,
        }
    }
}

struct NewConnection {
    user: Arc<Mutex<GatewayUser>>,
    client: Arc<Mutex<GatewayClient>>,
}

type ResumableClientsStore = Arc<Mutex<BTreeMap<String, DisconnectInfo>>>;
type GatewayUsersStore = Arc<Mutex<BTreeMap<Snowflake, Arc<Mutex<GatewayUser>>>>>;

pub async fn start_gateway(
    db: PgPool,
    publisher_map: SharedEventPublisherMap,
    config: Config,
) -> Result<(), Error> {
    // TODO(bitfl0wer): Add log messages throughout the method for debugging the gateway
    info!(target: "symfonia::gateway", "Starting gateway server");

    let bind = std::env::var("GATEWAY_BIND").unwrap_or_else(|_| String::from("localhost:3003"));
    let try_socket = TcpListener::bind(&bind).await;
    let listener = try_socket.expect("Failed to bind to address");

    info!(target: "symfonia::gateway", "Gateway server listening on port {bind}");

    let gateway_users: GatewayUsersStore = Arc::new(Mutex::new(BTreeMap::new()));
    let resumeable_clients: ResumableClientsStore = Arc::new(Mutex::new(BTreeMap::new()));
    tokio::task::spawn(async { purge_expired_disconnects(resumeable_clients) });
    while let Ok((stream, _)) = listener.accept().await {
        log::trace!(target: "symfonia::gateway", "New connection received");
        let connection_result = match tokio::task::spawn(
            establish_connection::establish_connection(stream, db.clone(), config.clone()),
        )
        .await
        {
            Ok(result) => result,
            Err(e) => {
                log::debug!(target: "symfonia::gateway::establish_connection", "User gateway task died: {e}");
                continue;
            }
        };
        match connection_result {
            Ok(new_connection) => checked_add_new_connection(gateway_users.clone(), new_connection),
            Err(e) => {
                log::debug!(target: "symfonia::gateway::establish_connection", "User gateway connection could not be established: {e}");
                continue;
            }
        }
    }
    Ok(())
}

/// A disconnected, resumable session can only be resumed within `RESUME_RECONNECT_WINDOW_SECONDS`
/// seconds after a disconnect occurs. Sessions that can be resumed are stored in a `Map`. The
/// purpose of this method is to periodically throw out expired sessions from that map.
fn purge_expired_disconnects(resumeable_clients: ResumableClientsStore) {
    let mut minutely_log_timer = 0;
    let mut removed_elements_last_minute: u128 = 0;
    loop {
        sleep(Duration::from_secs(5));
        // log::trace!(target: "symfonia::gateway::purge_expired_disconnects", "Removing stale disconnected sessions from list of resumeable sessions");
        let current_unix_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("Check the clock/time settings on the host machine")
            .as_secs();
        let mut to_remove = Vec::new();
        let mut lock = resumeable_clients.lock().unwrap();
        for (disconnected_session_id, disconnected_session_info) in lock.iter() {
            if current_unix_timestamp - disconnected_session_info.disconnected_at
                > RESUME_RECONNECT_WINDOW_SECONDS as u64
            {
                to_remove.push(disconnected_session_id.clone());
            }
        }
        let len = to_remove.len();
        removed_elements_last_minute = removed_elements_last_minute
            .checked_add(len as u128)
            .unwrap_or(u128::MAX);
        for session_id in to_remove.iter() {
            lock.remove(session_id);
        }
        drop(lock);
        minutely_log_timer += 1;
        if minutely_log_timer == 12 {
            log::debug!(target: "symfonia::gateway::purge_expired_disconnects", "Removed {} stale sessions in the last 60 seconds", removed_elements_last_minute);
            minutely_log_timer = 0;
            removed_elements_last_minute = 0;
        }
    }
}

/// Adds the contents of a [NewConnection] struct to a `gateway_users` map in a "checked" manner.
///
/// If the `NewConnection` contains a [GatewayUser] which is already in `gateway_users`, then
/// change the `parent` of the `NewConnection` [GatewayClient] to the User
/// from our `gateway_users` and push the client to the `clients` field of the User in our
/// `gateway_users``.
///
/// Else, add the [new GatewayUser] and the new [GatewayClient] into `gateway_users` as-is.
fn checked_add_new_connection(
    gateway_users: Arc<Mutex<BTreeMap<Snowflake, Arc<Mutex<GatewayUser>>>>>,
    new_connection: NewConnection,
) {
    // Make `new_connection` mutable
    let mut new_connection = new_connection;
    // To avoid having to get the lock a lot of times, lock once here and hold this lock for most
    // of the way through this method
    let new_connection_user = new_connection.user.lock().unwrap();
    let mut locked_map = gateway_users.lock().unwrap();
    // If our map contains the user from `new_connection` already, modify the `parent` of the `client`
    // of `new_connection` to point to the user already in our map, then insert that `client` into
    // the `clients` field of our existing user.
    if locked_map.contains_key(&new_connection_user.id) {
        let existing_user = locked_map.get(&new_connection_user.id).unwrap();
        new_connection.client.lock().unwrap().parent = Arc::downgrade(existing_user);
        existing_user
            .lock()
            .unwrap()
            .clients
            .push(new_connection.client);
    } else {
        // We cannot do `locked_map.insert(id, new_connection.user)` if new_connection is still
        // locked. Just bind the id we need to a new variable, then drop the lock.
        let id = new_connection_user.id;
        drop(new_connection_user);
        locked_map.insert(id, new_connection.user);
    }
}
