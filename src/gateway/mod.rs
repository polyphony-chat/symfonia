/*
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

static RESUME_RECONNECT_WINDOW_SECONDS: u8 = 90;

mod establish_connection;
mod gateway_task;
mod heartbeat;
mod types;

use std::collections::HashSet;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, Weak},
    thread::sleep,
    time::Duration,
};

use chorus::types::{
    GatewayHeartbeat, GatewayHello, GatewayIdentifyPayload, GatewayResume, Snowflake,
};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::info;
use pubserve::Subscriber;
use serde_json::{from_str, json};
use sqlx::PgPool;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};
pub use types::*;

use crate::database::entities::Config;
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::{
    errors::{Error, GatewayError},
    util::token::check_token,
    SharedEventPublisherMap, WebSocketReceive, WebSocketSend,
};

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

From there on, run a task that takes ownership of the Ga

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

Handling disconnects and session resumes is for late
*/

#[derive(Default, Clone)]
pub struct ConnectedUsers {
    store: Arc<Mutex<ConnectedUsersInner>>,
}

/// A mapping of Snowflake IDs to the "inbox" of a [GatewayUser].
///
/// An "inbox" is a [tokio::sync::mpsc::Sender] that can be used to send [Event]s to all connected
/// clients of a [GatewayUser].
#[derive(Default)]
pub struct ConnectedUsersInner {
    pub inboxes: HashMap<Snowflake, tokio::sync::broadcast::Sender<Event>>,
    pub users: HashMap<Snowflake, Arc<Mutex<GatewayUser>>>,
    pub resumeable_clients_store: ResumableClientsStore,
}

impl ConnectedUsers {
    /// Create a new, empty [ConnectedUsers] instance.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inner(&self) -> Arc<Mutex<ConnectedUsersInner>> {
        self.store.clone()
    }

    /// Register a new [GatewayUser] with the [ConnectedUsers] instance.
    async fn register(&self, user: GatewayUser) -> Arc<Mutex<GatewayUser>> {
        self.store
            .lock()
            .await
            .inboxes
            .insert(user.id, user.outbox.clone());
        let id = user.id;
        let arc = Arc::new(Mutex::new(user));
        self.store.lock().await.users.insert(id, arc.clone());
        arc
    }

    /// Deregister a [GatewayUser] from the [ConnectedUsers] instance.
    pub async fn deregister(&self, user: &GatewayUser) {
        self.store.lock().await.inboxes.remove(&user.id);
        self.store.lock().await.users.remove(&user.id);
    }

    /// Get the "inbox" of a [GatewayUser] by its Snowflake ID.
    pub async fn inbox(&self, id: Snowflake) -> Option<tokio::sync::broadcast::Sender<Event>> {
        self.store.lock().await.inboxes.get(&id).cloned()
    }

    pub async fn new_user(
        &self,
        clients: HashMap<String, Arc<Mutex<GatewayClient>>>,
        id: Snowflake,
        subscriptions: Vec<Box<dyn Subscriber<Event>>>,
    ) -> Arc<Mutex<GatewayUser>> {
        let channel = tokio::sync::broadcast::channel(20);
        let user = GatewayUser {
            inbox: channel.1,
            outbox: channel.0.clone(),
            clients,
            id,
            subscriptions,
            connected_users: self.clone(),
        };
        self.register(user).await
    }
}

/// A single identifiable User connected to the Gateway - possibly using many clients at the same
/// time.
struct GatewayUser {
    /// The "inbox" of a [GatewayUser]. This is a [tokio::sync::mpsc::Receiver]. Events sent to
    /// this inbox will be sent to all connected clients of this user.
    pub inbox: tokio::sync::broadcast::Receiver<Event>,
    /// The "outbox" of a [GatewayUser]. This is a [tokio::sync::mpsc::Sender]. From this outbox,
    /// more inboxes can be created.
    pub(super) outbox: tokio::sync::broadcast::Sender<Event>,
    /// Sessions a User is connected with. HashMap of SessionToken -> GatewayClient
    clients: HashMap<String, Arc<Mutex<GatewayClient>>>,
    /// The Snowflake ID of the User.
    id: Snowflake,
    /// A collection of [Subscribers](Subscriber) to [Event] [Publishers](pubserve::Publisher).
    ///
    /// A GatewayUser may have many [GatewayClients](GatewayClient), but he only gets subscribed to
    /// all relevant [Publishers](pubserve::Publisher) *once* to save resources.
    subscriptions: Vec<Box<dyn Subscriber<Event>>>,
    /// [Weak] reference to the [ConnectedUsers] store.
    connected_users: ConnectedUsers,
}

impl Hash for GatewayUser {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for GatewayUser {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for GatewayUser {}

/// A concrete session, that a [GatewayUser] is connected to the Gateway with.
struct GatewayClient {
    connection: Arc<Mutex<Connection>>,
    /// A [Weak] reference to the [GatewayUser] this client belongs to.
    parent: Weak<Mutex<GatewayUser>>,
    // Handle to the main Gateway task for this client
    main_task_handle: tokio::task::JoinHandle<()>,
    // Handle to the heartbeat task for this client
    heartbeat_task_handle: tokio::task::JoinHandle<()>,
    // Kill switch to disconnect the client
    kill_send: tokio::sync::broadcast::Sender<()>,
    /// Token of the session token used for this connection
    session_token: String,
    /// The last sequence number received from the client. Shared between the main task, heartbeat
    /// task, and this struct.
    last_sequence: Arc<Mutex<u64>>,
}

impl GatewayClient {
    pub async fn die(mut self, connected_users: ConnectedUsers) {
        self.kill_send.send(()).unwrap();
        let disconnect_info = DisconnectInfo {
            session_token: self.session_token.clone(),
            disconnected_at_sequence: *self.last_sequence.lock().await,
            parent: self.parent.clone(),
        };
        self.parent
            .upgrade()
            .unwrap()
            .lock()
            .await
            .clients
            .remove(&self.session_token);
        connected_users
            .deregister(self.parent.upgrade().unwrap().lock().await.deref())
            .await;
        connected_users
            .store
            .lock()
            .await
            .resumeable_clients_store
            .insert(self.session_token.clone(), disconnect_info);
    }
}

struct Connection {
    sender: WebSocketSend,
    receiver: WebSocketReceive,
}

#[derive(Clone)]
struct DisconnectInfo {
    /// session token that was used for this connection
    session_token: String,
    disconnected_at_sequence: u64,
    parent: Weak<Mutex<GatewayUser>>,
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

/// A map of resumable clients. The key is the session token used
/// for the connection. The value is a [GatewayClient] that can be resumed.
// TODO: this is stupid. it should be a map of string and DisconnectInfo. there is no need to store
// the whole GatewayClient, nor would it make sense to do so.
type ResumableClientsStore = HashMap<String, DisconnectInfo>;

pub async fn start_gateway(
    db: PgPool,
    connected_users: ConnectedUsers,
    config: Config,
) -> Result<(), Error> {
    // TODO(bitfl0wer): Add log messages throughout the method for debugging the gateway
    info!(target: "symfonia::gateway", "Starting gateway server");

    let bind = std::env::var("GATEWAY_BIND").unwrap_or_else(|_| String::from("localhost:3003"));
    let try_socket = TcpListener::bind(&bind).await;
    let listener = try_socket.expect("Failed to bind to address");

    info!(target: "symfonia::gateway", "Gateway server listening on port {bind}");

    let resumeable_clients: ResumableClientsStore = HashMap::new();
    let connected_users = ConnectedUsers::new();
    tokio::task::spawn(async { purge_expired_disconnects(connected_users.clone()).await });
    while let Ok((stream, _)) = listener.accept().await {
        log::trace!(target: "symfonia::gateway", "New connection received");
        let connection_result = match tokio::task::spawn(
            establish_connection::establish_connection(
                stream,
                db.clone(),
                config.clone(),
                connected_users.clone(),
            ),
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
            Ok(new_connection) => {
                checked_add_new_connection(connected_users.clone(), new_connection).await
            }
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
async fn purge_expired_disconnects(connected_users: ConnectedUsers) {
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
        let mut _inner = connected_users.inner();
        let mut lock = _inner.lock().await;
        for (disconnected_session_id, disconnected_session) in lock.resumeable_clients_store.iter()
        {
            // TODO(bitfl0wer): What are we calculating here? At least, this should be commented
            if current_unix_timestamp - disconnected_session.disconnected_at_sequence
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
            lock.resumeable_clients_store.remove(session_id);
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
async fn checked_add_new_connection(
    connected_users: ConnectedUsers,
    new_connection: NewConnection,
) {
    // Make `new_connection` mutable
    let mut new_connection = new_connection;
    // To avoid having to get the lock a lot of times, lock once here and hold this lock for most
    // of the way through this method
    let new_connection_user = new_connection.user.lock().await;
    let new_connection_token = new_connection.client.lock().await.session_token.clone();
    let mut lock = connected_users.inner().lock().await;
    // If our map contains the user from `new_connection` already, modify the `parent` of the `client`
    // of `new_connection` to point to the user already in our map, then insert that `client` into
    // the `clients` field of our existing user.
    if lock.inner.contains_key(&new_connection_user.id) {
        let existing_user = lock.inner.get(&new_connection_user.id).unwrap();
        new_connection.client.lock().await.parent = Arc::downgrade(existing_user);
        existing_user
            .lock()
            .await
            .clients
            .insert(new_connection_token, new_connection.client);
    } else {
        // We cannot do `locked_map.insert(id, new_connection.user)` if new_connection is still
        // locked. Just bind the id we need to a new variable, then drop the lock.
        let id = new_connection_user.id;
        drop(new_connection_user);
        locked_map.insert(id, new_connection.user);
    }
}
