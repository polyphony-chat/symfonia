// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use chorus::types::{GatewayHeartbeat, Snowflake};
use log::debug;
use serde_json::json;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::{
	Message,
	protocol::{CloseFrame, frame::coding::CloseCode},
};
use util::{
	errors::{Error, GatewayError},
	gateway::{WebSocketConnection, event::Event},
};

use super::ConnectedUsers;

/// Handles all messages a client sends to the gateway post-handshake.
pub(super) async fn gateway_task(
	mut connection: WebSocketConnection,
	inbox: tokio::sync::broadcast::Receiver<Event>,
	heartbeat_send: tokio::sync::broadcast::Sender<GatewayHeartbeat>,
	last_sequence_number: Arc<Mutex<u64>>,
	connected_users: ConnectedUsers,
	user_id: Snowflake,
) {
	log::trace!(target: "symfonia::gateway::gateway_task", "Started a new gateway task!");
	let inbox_processor = tokio::spawn(process_inbox(connection.clone(), inbox.resubscribe()));

	/*
	Before we can respond to any gateway event we receive, we need to figure out what kind of event
	we are dealing with. For a lot of events, this is easy, because we can just look at the opcode
	and figure out the event type. For the dispatch events however, we also need to look at the event
	name to find out the exact dispatch event we are dealing with. -bitfl0wer
	 */

	loop {
		tokio::select! {
			_ = connection.kill_receive.recv() => {
				// Since callsites handle closing the connection, we don't need to do that here.
				// Perform cleanup and return
				let mut store_lock = connected_users.store.write();
				store_lock.users.remove(&user_id);
				store_lock.inboxes.remove(&user_id);
				// TODO(bitfl0wer) Add the user to the disconnected sessions
				drop(store_lock);
				return;
			},
			message_result = connection.receiver.recv() => {
				if message_result.is_err() {
					connection.sender.send(Message::Close(Some(CloseFrame { code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Library(4000), reason: "INTERNAL_SERVER_ERROR".into() })));
					connection.kill_send.send(()).expect("Failed to send kill_send");
				}
				let message_of_unknown_type = message_result.unwrap();
				match message_of_unknown_type {
					Message::Text(_) => {
						log::trace!(target: "symfonia::gateway::gateway_task", "Received raw message {:?}", message_of_unknown_type);
						let event = unwrap_event(Event::try_from(message_of_unknown_type), connection.clone(), connection.kill_send.clone());
						handle_event(event, connection.clone(), heartbeat_send.clone());
					},
					Message::Close(close_frame) => {
						// Closing is initiated by the client - we don't need to send a
						// close message back.
						debug!("Client is closing connection. Signaling gateway_task to shut down");
						connection.kill_send.send(());
					},
					_ => continue
				}
			}
		}
	}
}

/// Handle an event received from the gateway.
fn handle_event(
	event: Event,
	connection: WebSocketConnection,
	heartbeat_send: tokio::sync::broadcast::Sender<GatewayHeartbeat>,
) {
	log::trace!(target: "symfonia::gateway::gateway_task", "Event type of received message: {:?}", event);
	match event {
		Event::Dispatch(_) => {
			// Receiving a dispatch event from a client is never correct
			log::debug!(target: "symfonia::gateway::gateway_task", "Received an unexpected message: {:?}", event);
			connection.sender.send(Message::Close(Some(CloseFrame {
				code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Library(
					4002,
				),
				reason: "DECODE_ERROR".into(),
			})));
			connection.kill_send.send(()).expect("Failed to send kill_send");
		}
		Event::Heartbeat(hearbeat_event) => {
			match heartbeat_send.send(hearbeat_event) {
				Err(e) => {
					log::debug!(target: "symfonia::gateway::gateway_task", "Received Heartbeat but HeartbeatHandler seems to be dead?");
					connection.sender.send(Message::Close(Some(CloseFrame { code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Library(4002), reason: "DECODE_ERROR".into() })));
					connection.kill_send.send(()).expect("Failed to send kill_send");
				}
				Ok(_) => {
					log::trace!(target: "symfonia::gateway::gateway_task", "Forwarded heartbeat message to HeartbeatHandler!");
				}
			}
		}
		_ => {
			log::error!(target: "symfonia::gateway::gateway_task", "Received an event type for which no code is yet implemented in the gateway_task. Please open a issue or PR at the symfonia repository. {:?}", event);
		}
	}
}

/// Unwraps an event from a Result<Event, Error> and handles the error if there
/// is one. Errors will shut down all tasks belonging to this session and will
/// kill the gateway task through a panic.
fn unwrap_event(
	result: Result<Event, Error>,
	connection: WebSocketConnection,
	kill_send: tokio::sync::broadcast::Sender<()>,
) -> Event {
	match result {
		Err(e) => {
			match e {
				Error::Gateway(g) => match g {
					GatewayError::UnexpectedOpcode(o) => {
						log::debug!(target: "symfonia::gateway::gateway_task::unwrap_event", "Received an unexpected opcode: {:?}", o);
						connection.sender.send(Message::Close(Some(CloseFrame { code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Library(4001), reason: "UNKNOWN_OPCODE".into() })));
						kill_send.send(()).expect("Failed to send kill_send");
						panic!("Killing gateway task: Received an unexpected opcode");
					}
					GatewayError::UnexpectedMessage(m) => {
						log::debug!(target: "symfonia::gateway::gateway_task::unwrap_event", "Received an unexpected message: {:?}", m);
						connection.sender.send(Message::Close(Some(CloseFrame { code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Library(4002), reason: "DECODE_ERROR".into() })));
						kill_send.send(()).expect("Failed to send kill_send");
						panic!("Killing gateway task: Received an unexpected message");
					}
					_ => {
						log::debug!(target: "symfonia::gateway::gateway_task::unwrap_event", "Received an unexpected error: {:?}", g);
						connection.sender.send(Message::Close(Some(CloseFrame { code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Library(4000), reason: "INTERNAL_SERVER_ERROR".into() })));
						kill_send.send(()).expect("Failed to send kill_send");
						panic!("Killing gateway task: Received an unexpected error");
					}
				},
				_ => {
					log::debug!(target: "symfonia::gateway::gateway_task::unwrap_event", "Received an unexpected error: {:?}", e);
					connection.sender.send(Message::Close(Some(CloseFrame { code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Library(4000), reason: "INTERNAL_SERVER_ERROR".into() })));
					kill_send.send(()).expect("Failed to send kill_send");
					panic!("Killing gateway task: Received an unexpected error");
				}
			}
		}
		Ok(event) => event,
	}
}

/// Process events triggered by the HTTP API.
async fn process_inbox(
	mut connection: WebSocketConnection,
	mut inbox: tokio::sync::broadcast::Receiver<Event>,
) {
	loop {
		tokio::select! {
			_ = connection.kill_receive.recv() => {
				return;
			}
			event = inbox.recv() => {
				match event {
					Ok(event) => {
						let send_result = connection.sender.send(Message::Text(json!(event).to_string().into()));
						match send_result {
							Ok(_) => (), // TODO: Increase sequence number here
							Err(_) => {
								debug!("Failed to send event to WebSocket. Closing connection and killing tasks");
								connection.sender.send(Message::Close(Some(CloseFrame { code: CloseCode::Library(4000), reason: "WebSocket error".into() })));
								connection.kill_send.send(()).expect("Failed to send kill_send");
							},
						}
					}
					Err(_) => {
						return;
					}
				}
			}
		}
	}
}
