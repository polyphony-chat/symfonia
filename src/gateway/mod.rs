mod types;

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, Weak};

use chorus::types::{GatewayIdentifyPayload, Snowflake};
use log::info;
use pubserve::Subscriber;
use sqlx::MySqlPool;
use tokio::net::{TcpListener, TcpStream};

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

Handling disconnects and session resumes is for later, I think.
*/

/// A single identifiable User connected to the Gateway - possibly using many clients at the same
/// time.
struct GatewayUser {
    /// Sessions a User is connected with.
    clients: Vec<GatewayClient>,
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
    /// A [Weak] reference to the [GatewayUser] this client belongs to.
    parent: Weak<GatewayUser>,
    /// The [TcpStream] for this WebSocket session
    connection: TcpStream,
    /// [GatewayIdentifyPayload] the client has sent when connecting (or re-connecting) with this client.
    identify: GatewayIdentifyPayload,
}

struct NewConnection {
    user: Option<GatewayUser>,
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

    let mut gateway_users: Arc<Mutex<BTreeMap<Snowflake, GatewayUser>>> =
        Arc::new(Mutex::new(BTreeMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        tokio::task::spawn(establish_connection(stream));
    }
    Ok(())
}

async fn establish_connection(stream: TcpStream) -> Result<NewConnection, Error> {
    Ok(())
}
