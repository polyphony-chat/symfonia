use std::sync::Arc;

use tokio::sync::Mutex;

use super::GatewayClient;

/// Handles all messages a client sends to the gateway post-handshake.
pub(super) async fn gateway_task(connection: Arc<Mutex<super::Connection>>) {
    // TODO
    todo!()
}
