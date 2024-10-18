/*
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

static RESUME_RECONNECT_WINDOW_SECONDS: u8 = 90;
static DEFAULT_GATEWAY_BIND: &str = "0.0.0.0:3003";

mod establish_connection;
mod gateway_task;
mod heartbeat;
mod ready;
mod types;

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::Hash,
    ops::{Deref, DerefMut},
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
use sqlx_pg_uint::PgU64;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, MutexGuard},
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

/// A map of resumable clients. The key is the session token used
/// for the connection. The value is a [GatewayClient] that can be resumed.
// TODO: this is stupid. it should be a map of string and DisconnectInfo. there is no need to store
// the whole GatewayClient, nor would it make sense to do so.
pub type ResumableClientsStore = HashMap<String, DisconnectInfo>;

pub async fn start_gateway(
    db: PgPool,
    connected_users: ConnectedUsers,
    config: Config,
) -> Result<(), Error> {
    // TODO(bitfl0wer): Add log messages throughout the method for debugging the gateway
    info!(target: "symfonia::gateway", "Starting gateway server");

    let bind = &std::env::var("GATEWAY_BIND").unwrap_or_else(|_| {
        log::warn!(target: "symfonia::db", "You did not specify GATEWAY_BIND environment variable. Defaulting to '{DEFAULT_GATEWAY_BIND}'.");
        DEFAULT_GATEWAY_BIND.to_string()
    });
    let try_socket = TcpListener::bind(&bind).await;
    let listener = try_socket.expect("Failed to bind to address");

    info!(target: "symfonia::gateway", "Gateway server listening on port {bind}");

    let resumeable_clients: ResumableClientsStore = HashMap::new();
    let connected_users_clone = connected_users.clone();
    tokio::task::spawn(async { purge_expired_disconnects(connected_users_clone).await });
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
            Ok(_) => (),
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
        let mut read_lock = _inner.read();
        for (disconnected_session_id, disconnected_session) in
            read_lock.resumeable_clients_store.iter()
        {
            // TODO(bitfl0wer): What are we calculating here? At least, this should be commented
            if current_unix_timestamp - disconnected_session.disconnected_at_sequence
                > RESUME_RECONNECT_WINDOW_SECONDS as u64
            {
                to_remove.push(disconnected_session_id.clone());
            }
        }
        drop(read_lock);
        let len = to_remove.len();
        removed_elements_last_minute = removed_elements_last_minute
            .checked_add(len as u128)
            .unwrap_or(u128::MAX);
        let mut write_lock = _inner.write();
        for session_id in to_remove.iter() {
            write_lock.resumeable_clients_store.remove(session_id);
        }
        drop(write_lock);
        minutely_log_timer += 1;
        if minutely_log_timer == 12 {
            log::debug!(target: "symfonia::gateway::purge_expired_disconnects", "Removed {} stale sessions in the last 60 seconds", removed_elements_last_minute);
            minutely_log_timer = 0;
            removed_elements_last_minute = 0;
        }
    }
}
