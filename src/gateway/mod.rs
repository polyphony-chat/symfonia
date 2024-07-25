mod types;

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, Weak};

use chorus::types::{GatewayIdentifyPayload, Snowflake};
use futures::stream::{SplitSink, SplitStream};
use futures::StreamExt;
use log::info;
use pubserve::Subscriber;
use sqlx::MySqlPool;
use tokio::net::{TcpListener, TcpStream};

use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{accept_async, WebSocketStream};
pub use types::*;

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::errors::Error;
use crate::SharedEventPublisherMap;

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
    pub clients: Vec<GatewayClient>,
    /// The Snowflake ID of the User.
    pub id: Snowflake,
    /// A collection of [Subscribers](Subscriber) to [Event] [Publishers](pubserve::Publisher).
    ///
    /// A GatewayUser may have many [GatewayClients](GatewayClient), but he only gets subscribed to
    /// all relevant [Publishers](pubserve::Publisher) *once* to save resources.
    pub subscriptions: Vec<Box<dyn Subscriber<Event>>>,
}

/// A concrete session, that a [GatewayUser] is connected to the Gateway with.
struct GatewayClient {
    /// A [Weak] reference to the [GatewayUser] this client belongs to.
    pub parent: Weak<Mutex<GatewayUser>>,
    /// The [SplitSink] and [SplitStream] for this clients' WebSocket session
    pub connection: (
        SplitSink<WebSocketStream<TcpStream>, Message>,
        SplitStream<WebSocketStream<TcpStream>>,
    ),
    /// [GatewayIdentifyPayload] the client has sent when connecting (or re-connecting) with this client.
    pub identify: GatewayIdentifyPayload,
}

struct NewConnection {
    user: Arc<Mutex<GatewayUser>>,
    client: GatewayClient,
}

pub async fn start_gateway(
    db: MySqlPool,
    publisher_map: SharedEventPublisherMap,
) -> Result<(), Error> {
    info!(target: "symfonia::gateway", "Starting gateway server");

    let bind = std::env::var("GATEWAY_BIND").unwrap_or_else(|_| String::from("localhost:3003"));
    let try_socket = TcpListener::bind(&bind).await;
    let listener = try_socket.expect("Failed to bind to address");

    let gateway_users: Arc<Mutex<BTreeMap<Snowflake, Arc<Mutex<GatewayUser>>>>> =
        Arc::new(Mutex::new(BTreeMap::new()));
    while let Ok((stream, _)) = listener.accept().await {
        let connection_result = match tokio::task::spawn(establish_connection(stream)).await {
            Ok(result) => result,
            Err(_) => continue,
        };
        match connection_result {
            Ok(new_connection) => checked_add_new_connection(gateway_users.clone(), new_connection),
            Err(_) => todo!(),
        }
    }
    Ok(())
}

/// Handle the Gateway connection initalization process of a client connecting to the Gateway up
/// until the point where we receive the identify payload, at which point a [NewConnection] will
/// be returned.
///
/// If successful, returns a [NewConnection] with a new [Arc<Mutex<GatewayUser>>] and a
/// [GatewayClient], whose `.parent` field contains a [Weak] reference to the new [GatewayUser].
async fn establish_connection(stream: TcpStream) -> Result<NewConnection, Error> {
    let ws_stream = accept_async(stream).await?;
    let (write, read) = ws_stream.split();

    // TODO: Everything below this is just an example and absolutely unfinished.
    let user = Arc::new(Mutex::new(GatewayUser {
        clients: Vec::new(),
        id: Snowflake(1),
        subscriptions: Vec::new(),
    }));
    Ok(NewConnection {
        user: user.clone(),
        client: GatewayClient {
            parent: Arc::downgrade(&user),
            connection: (write, read),
            identify: GatewayIdentifyPayload::common(),
        },
    })
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
        new_connection.client.parent = Arc::downgrade(existing_user);
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
