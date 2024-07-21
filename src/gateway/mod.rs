mod types;

use log::info;
use pubserve::Publisher;
pub use types::*;

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::errors::Error;

pub type EventPublisher = Publisher<Event>;

pub async fn start_gateway() -> Result<(), Error> {
    info!(target: "symfonia::gateway", "Starting gateway server");
    Ok(())
}
