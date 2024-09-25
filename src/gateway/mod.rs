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
use sqlx_pg_uint::PgU64;
use tokio::sync::MutexGuard;
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

#[derive(Default)]
/// Represents all existing roles on the server and the users that have these roles.
pub struct RoleUserMap {
    /// Map Role Snowflake ID to a list of User Snowflake IDs
    map: HashMap<Snowflake, HashSet<Snowflake>>,
}

impl Deref for RoleUserMap {
    type Target = HashMap<Snowflake, HashSet<Snowflake>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for RoleUserMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl RoleUserMap {
    /// Initialize the [RoleUserMap] with data from the database.
    ///
    /// This method will query the database for all roles and all users that have these roles.
    /// The data will then populate the map.
    ///
    /// Due to the possibly large number of roles and users returned by the database, this method
    /// should only be executed once. The [RoleUserMap] should be kept synchronized with the database
    /// through means that do not involve this method.
    pub async fn init(&mut self, db: &PgPool) -> Result<(), Error> {
        // First, get all role ids from the roles table and insert them into the map
        let all_role_ids: Vec<PgU64> = sqlx::query_as("SELECT id FROM roles")
            .fetch_all(db)
            .await
            .map_err(Error::SQLX)?;
        for role_id in all_role_ids.iter() {
            self.map
                .insert(Snowflake::from(role_id.to_uint()), HashSet::new());
        }
        // Then, query member_roles and insert the user ids into the map
        let all_member_roles: Vec<(PgU64, PgU64)> =
            sqlx::query_as("SELECT index, role_id FROM member_roles")
                .fetch_all(db)
                .await
                .map_err(Error::SQLX)?;
        for (user_id, role_id) in all_member_roles.iter() {
            // Unwrapping is fine here, as the member_roles table has a foreign key constraint
            // which states that role_id must be a valid id in the roles table.
            let users_for_role_id = self.map.get_mut(&role_id.to_uint().into()).unwrap();
            users_for_role_id.insert(user_id.to_uint().into());
        }
        Ok(())
    }
}

pub struct Connection {
    sender: WebSocketSend,
    receiver: WebSocketReceive,
}

#[derive(Clone)]
pub struct DisconnectInfo {
    /// session token that was used for this connection
    pub session_token: String,
    pub disconnected_at_sequence: u64,
    pub parent: Weak<Mutex<GatewayUser>>,
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

/// Represents a new successful connection to the gateway. The user is already part of the [ConnectedUsers]
/// and the client is already registered with the [GatewayClient] "clients" map.
struct NewConnection {
    user: Arc<Mutex<GatewayUser>>,
    client: Arc<Mutex<GatewayClient>>,
}

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

    let bind = std::env::var("GATEWAY_BIND").unwrap_or_else(|_| String::from("localhost:3003"));
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
