mod events;
mod types;

use chorus::types::Snowflake;
use serde::{Deserialize, Serialize};

pub use events::*;
pub use types::*;

use log::info;

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::errors::Error;

pub async fn start_gateway() -> Result<(), Error> {
    info!(target: "symfonia::gateway", "Starting gateway server");
    Ok(())
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, Deserialize, Serialize)]
/// Identifies a unique event emitter with an event type and a snowflake ID.
pub struct EventEmitter {
    pub event_type: EventType,
    pub id: Snowflake,
}
