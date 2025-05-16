// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
mod instance;
mod stats;

use poem::{Route, get};
pub use stats::*;

pub fn setup_routes() -> Route {
	Route::new().nest("/instance", instance::setup_routes()).at("/stats", get(stats))
}
