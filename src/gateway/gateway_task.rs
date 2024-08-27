use std::sync::{Arc, Mutex};

use super::GatewayClient;

/// Handles all messages a client sends to the gateway post-handshake.
pub(super) async fn gateway_task(
    client: Arc<Mutex<GatewayClient>>,
) -> Result<(), crate::errors::Error> {
    // TODO
    todo!()
}
