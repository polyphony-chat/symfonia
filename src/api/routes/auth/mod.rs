mod login;
mod register;

pub use login::*;
use poem::{post, Route};
pub use register::*;

pub fn setup_routes() -> Route {
    Route::new()
        .at("/login", post(login))
        .at("/register", post(register))
}
