mod types;

use log::info;
use sqlx::MySqlPool;
pub use types::*;

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::errors::Error;
use crate::SharedEventPublisherMap;

pub async fn start_gateway(
    db: MySqlPool,
    publisher_map: SharedEventPublisherMap,
) -> Result<(), Error> {
    info!(target: "symfonia::gateway", "Starting gateway server");
    // `publishers` will live for the lifetime of the gateway server, in the main gateway thread
    Ok(())
}
