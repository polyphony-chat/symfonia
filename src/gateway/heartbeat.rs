use std::sync::Arc;

use chorus::types::{GatewayHeartbeat, GatewayHeartbeatAck};
use futures::SinkExt;
use log::*;
use rand::seq;
use serde_json::json;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

use super::{Connection, GatewayClient};

static HEARTBEAT_INTERVAL: std::time::Duration = std::time::Duration::from_secs(45);
static LATENCY_BUFFER: std::time::Duration = std::time::Duration::from_secs(5);

pub(super) struct HeartbeatHandler {
    connection: Arc<Mutex<Connection>>,
    kill_receive: tokio::sync::broadcast::Receiver<()>,
    kill_send: tokio::sync::broadcast::Sender<()>,
    message_receive: tokio::sync::mpsc::Receiver<GatewayHeartbeat>,
    last_heartbeat: std::time::Instant,
    /// The current sequence number of the gateway connection.
    sequence_number: Arc<Mutex<u64>>,
}

impl HeartbeatHandler {
    /// Constructs a new `HeartbeatHandler` instance.
    ///
    /// This method initializes a new heartbeat handler with the provided connection, kill signals, and message receiver. It sets up the internal state for tracking the last heartbeat time.
    ///
    /// # Parameters
    /// - `connection`: A shared reference to a mutex-protected connection object.
    /// - `kill_receive`: A channel receiver for signaling the shutdown of the heartbeat handler.
    /// - `kill_send`: A channel sender for sending signals to shut down the heartbeat handler.
    /// - `message_receive`: An MPSC (Multiple Producer Single Consumer) channel receiver for receiving heartbeat messages.
    ///
    /// # Returns
    /// The newly created `HeartbeatHandler` instance.
    ///
    /// # Example
    /// ```rust
    /// use std::sync::Arc;
    /// use tokio::sync::broadcast;
    /// use tokio::sync::mpsc;
    /// use chorus::types::GatewayHeartbeat;
    /// use super::Connection;
    /// use super::HeartbeatHandler;
    ///
    /// let connection = Arc::new(Mutex::new(Connection::new()));
    /// let (kill_send, kill_receive) = broadcast::channel(1);
    /// let (message_send, message_receive) = mpsc::channel(16);
    ///
    /// let heartbeat_handler = HeartbeatHandler::new(connection, kill_receive, kill_send, message_receive).await;
    /// ```
    pub(super) fn new(
        connection: Arc<Mutex<Connection>>,
        kill_receive: tokio::sync::broadcast::Receiver<()>,
        kill_send: tokio::sync::broadcast::Sender<()>,
        message_receive: tokio::sync::mpsc::Receiver<GatewayHeartbeat>,
        sequence_number: Arc<Mutex<u64>>,
    ) -> Self {
        Self {
            connection,
            kill_receive,
            kill_send,
            message_receive,
            last_heartbeat: std::time::Instant::now(),
            sequence_number,
        }
    }

    /// Continuously listens for messages and handles heartbeat logic until instructed to shut down.
    ///
    /// This asynchronous method maintains an infinite loop that waits for signals to either receive
    /// a new heartbeat message or check if it should terminate. It updates the last heartbeat time
    /// upon receiving a new heartbeat, sends a ping over the WebSocket connection periodically, and
    /// terminates itself if no heartbeats are received within 45 seconds. Because this method is
    /// running an "infinite" loop, the [HeartbeatHandler] should be moved to a separate task using
    /// `tokio::spawn`, where the method should be executed.
    ///
    /// ## Termination
    /// The loop terminates when:
    /// - A shutdown signal is received through `kill_receive`.
    /// - An error occurs during WebSocket communication or channel reception.
    ///
    /// Termination is signaled by sending a message through `kill_send` to the main task. This
    /// `kill_send` channel is created by the main task and passed to the `HeartbeatHandler` during
    /// initialization. The corresponding `kill_receive` can be used by other tasks to signal that
    /// the Gateway connection should be closed. In the context of symfonia, this is being done to
    /// close the [GatewayTask].
    ///
    ///
    /// ## Example
    /// ```rust
    /// use std::sync::Arc;
    /// use tokio::sync::broadcast;
    /// use tokio::sync::mpsc;
    /// use chorus::types::GatewayHeartbeat;
    /// use super::Connection;
    /// use super::HeartbeatHandler;
    ///
    /// let connection = Arc::new(Mutex::new(Connection::new()));
    /// let (kill_send, kill_receive) = broadcast::channel(1);
    /// let (message_send, message_receive) = mpsc::channel(16);
    ///
    /// let mut handler = HeartbeatHandler::new(connection, kill_receive, kill_send, message_receive).await;
    /// tokio::spawn(async move {
    ///     handler.run();
    /// });
    /// ```
    pub(super) async fn run(&mut self) {
        // TODO: On death of this task, create and store disconnect info in gateway client object
        let mut sequence = 0u64;
        loop {
            // When receiving heartbeats, we need to consider the following cases:
            // - Heartbeat sequence number is correct
            // - Heartbeat sequence number is slightly off, likely because a new packet was sent before the heartbeat was received
            // - Heartbeat sequence number is way off, likely because the connection has high latency or is unstable
            //
            // I would consider "way off" to be a difference of more than or equal to 3.
            tokio::select! {
                _ = self.kill_receive.recv() => {
                    trace!("Received kill signal in heartbeat_handler. Stopping heartbeat handler");
                    break;
                }
                Some(heartbeat) = self.message_receive.recv() => {
                    trace!("Received heartbeat message in heartbeat_handler");
                    if let Some(received_sequence_number) = heartbeat.d {
                        let mut sequence = self.sequence_number.lock().await;
                        // TODO: ..wait do we actually even *receive* sequence numbers, or do we just send them?
                        match *sequence + 1 == received_sequence_number {
                            true => {
                                *sequence = received_sequence_number;
                            }
                            false => {
                                // TODO Send disconnect message
                                self.connection.lock().await.sender.send().await.unwrap();
                                self.kill_send.send(()).unwrap();
                                break;
                            }
                        }
                    }
                    self.last_heartbeat = std::time::Instant::now();
                    self.connection
                .lock()
                .await
                .sender
                .send(Message::Text(
                    json!(GatewayHeartbeatAck::default()).to_string(),
                ))
                .await.unwrap_or_else(|_| {
                    trace!("Failed to send heartbeat ack in heartbeat_handler. Stopping gateway_task and heartbeat_handler");
                    self.kill_send.send(()).expect("Failed to send kill signal in heartbeat_handler");
                    return;
                }
                );
                }
                else => {
                    let elapsed = std::time::Instant::now() - self.last_heartbeat;
                    if elapsed > std::time::Duration::from_secs(45) {
                        trace!("Heartbeat timed out in heartbeat_handler. Stopping gateway_task and heartbeat_handler");
                        self.kill_send.send(()).expect("Failed to send kill signal in heartbeat_handler");;
                        break;
                    }
                }
            }
        }
    }
}