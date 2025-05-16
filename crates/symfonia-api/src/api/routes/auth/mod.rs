// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
mod login;
mod register;

pub use login::*;
use poem::{Route, post};
pub use register::*;

pub fn setup_routes() -> Route {
	Route::new().at("/login", post(login)).at("/register", post(register))
}
