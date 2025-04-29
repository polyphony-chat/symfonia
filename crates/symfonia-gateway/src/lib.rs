mod establish_connection;
mod gateway_task;
mod heartbeat;
mod ready;

static RESUME_RECONNECT_WINDOW_SECONDS: u8 = 90;
static DEFAULT_GATEWAY_BIND: &str = "0.0.0.0:3003";

use std::{collections::HashMap, thread::sleep, time::Duration};

use log::info;
use sqlx::PgPool;
use tokio::net::TcpListener;
use util::{
	configuration::SymfoniaConfiguration,
	entities::Config,
	errors::Error,
	gateway::{ConnectedUsers, ResumableClientsStore},
};

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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

pub async fn start_gateway(
	db: PgPool,
	connected_users: ConnectedUsers,
	config: Config,
) -> Result<(), Error> {
	// TODO(bitfl0wer): Add log messages throughout the method for debugging the
	// gateway
	info!(target: "symfonia::gateway", "Starting gateway server");

	let port = SymfoniaConfiguration::get().gateway.cfg.port;
	let host = &SymfoniaConfiguration::get().gateway.cfg.host;
	// .trim() needs to be called because \n is appended to the .to_string(),
	// messing up the binding
	let try_socket = TcpListener::bind((host.as_str(), port)).await;
	let listener = try_socket.expect("Failed to bind to address");

	info!(target: "symfonia::gateway", "Gateway server listening on {host}:{port}");

	let resumeable_clients: ResumableClientsStore = HashMap::new();
	let connected_users_clone = connected_users.clone();
	tokio::task::spawn(async { purge_expired_disconnects(connected_users_clone).await });
	while let Ok((stream, _)) = listener.accept().await {
		log::trace!(target: "symfonia::gateway", "New connection received");
		let connection_result =
			match tokio::task::spawn(establish_connection::establish_connection(
				stream,
				db.clone(),
				config.clone(),
				connected_users.clone(),
			))
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

/// A disconnected, resumable session can only be resumed within
/// `RESUME_RECONNECT_WINDOW_SECONDS` seconds after a disconnect occurs.
/// Sessions that can be resumed are stored in a `Map`. The purpose of this
/// method is to periodically throw out expired sessions from that map.
async fn purge_expired_disconnects(connected_users: ConnectedUsers) {
	let mut minutely_log_timer = 0;
	let mut removed_elements_last_minute: u128 = 0;
	loop {
		sleep(Duration::from_secs(5));
		// log::trace!(target: "symfonia::gateway::purge_expired_disconnects", "Removing
		// stale disconnected sessions from list of resumeable sessions");
		let current_unix_timestamp = std::time::SystemTime::now()
			.duration_since(std::time::SystemTime::UNIX_EPOCH)
			.expect("Check the clock/time settings on the host machine")
			.as_secs();
		let mut to_remove = Vec::new();
		let mut _inner = connected_users.inner();
		let read_lock = _inner.read();
		for (disconnected_session_id, disconnected_session) in
			read_lock.resumeable_clients_store.iter()
		{
			// TODO(bitfl0wer): What are we calculating here? At least, this should be
			// commented
			if current_unix_timestamp - disconnected_session.disconnected_at_sequence
				> RESUME_RECONNECT_WINDOW_SECONDS as u64
			{
				to_remove.push(disconnected_session_id.clone());
			}
		}
		drop(read_lock);
		let len = to_remove.len();
		removed_elements_last_minute =
			removed_elements_last_minute.checked_add(len as u128).unwrap_or(u128::MAX);
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

/// Tells every user-/client specific tokio task spawned by the symfonia binary
/// to yield so that the server may shut down in an orderly fashion.
///
/// ## #\[allow(clippy::await_holding_lock)]
///
/// We hold the lock of `inner.write()` across an await point
/// `user_mutex.lock().await`. The lock is held across the await point right
/// before shutting down the application. It should be okay to do this. The
/// inevitable shutdown of the application should guarantee no contention.
///
/// TODO: This is currently unused. We cannot use this, as the future created by
/// this function is not `Send`. This is because of the whole "holding mutex
/// across await" thing. We need to find a better solution for this.
#[allow(clippy::await_holding_lock)]
pub async fn tokio_task_killer(connected_users: ConnectedUsers) {
	exit_signal_detected().await;
	log::debug!("Exit signal detected!");
	let inner = connected_users.inner();
	let users = &mut inner.write().users;
	for (_, user_mutex) in users.iter() {
		let mut user = user_mutex.lock().await;
		user.kill().await;
	}
}

/// Detects when an exit signal is sent by the operating system. The future will
/// complete when an exit signal is detected.
async fn exit_signal_detected() {
	#[cfg(all(unix, windows))]
	{
		panic!("Unsupported platform; How did you get here?");
	}

	#[cfg(unix)]
	{
		// All these signals should shut down an application on UNIX-like systems
		use tokio::signal::unix::{SignalKind, signal};
		let mut sig_alarm = signal(SignalKind::alarm()).unwrap();
		let mut sig_hangup = signal(SignalKind::hangup()).unwrap();
		let mut sig_interrupt = signal(SignalKind::interrupt()).unwrap();
		let mut sig_pipe = signal(SignalKind::interrupt()).unwrap();
		let mut sig_quit = signal(SignalKind::quit()).unwrap();
		let mut sig_terminate = signal(SignalKind::terminate()).unwrap();
		let mut sig_user_defined1 = signal(SignalKind::user_defined1()).unwrap();
		let mut sig_user_defined2 = signal(SignalKind::user_defined2()).unwrap();
		let ctrl_c = tokio::signal::ctrl_c();

		tokio::select! {
			// If we receive any of these signals, yield
			_ = sig_alarm.recv() => (),
			_ = sig_hangup.recv() => (),
			_ = sig_interrupt.recv() => (),
			_ = sig_pipe.recv() => (),
			_ = sig_quit.recv() => (),
			_ = sig_terminate.recv() => (),
			_ = sig_user_defined1.recv() => (),
			_ = sig_user_defined2.recv() => (),
			event = ctrl_c => event.expect("Failed to listen to CTRL-c event"),
		}
	}

	#[cfg(windows)]
	{
		// All these signals should shut down an application on Windows
		use tokio::signal::windows::{ctrl_break, ctrl_close, ctrl_logoff, ctrl_shutdown};
		let mut sig_break = ctrl_break().unwrap();
		let ctrl_c = tokio::signal::ctrl_c();
		let mut sig_close = ctrl_close().unwrap();
		let mut sig_logoff = ctrl_logoff().unwrap();
		let mut sig_shutdown = ctrl_shutdown().unwrap();

		tokio::select! {
			// If we receive any of these signals, yield
			_ = sig_break.recv() => (),
			event = ctrl_c => event.expect("Failed to listen to CTRL-c event"),
			_ = sig_close.recv() => (),
			_ = sig_logoff.recv() => (),
			_ = sig_shutdown.recv() => (),
		}
	}

	#[cfg(not(any(unix, windows)))]
	{
		panic!("Unsupported platform");
	}
}
