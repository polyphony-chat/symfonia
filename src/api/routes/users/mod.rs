use poem::Route;

mod me;

pub fn setup_routes() -> Route {
    Route::new().nest("/@me", me::setup_routes())
}
