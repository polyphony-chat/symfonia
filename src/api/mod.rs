use poem::{EndpointExt, Route};

use crate::api::{
    middleware::{authentication::AuthenticationMiddleware, current_user::CurrentUserMiddleware},
    routes::{auth, channels, guilds, users},
};
use crate::PathRouteTuple;

mod middleware;
mod routes;

pub fn setup_api() -> PathRouteTuple {
    let routes = Route::new()
        .nest("/auth", auth::setup_routes())
        .nest(
            "/users",
            users::setup_routes()
                .with(AuthenticationMiddleware)
                .with(CurrentUserMiddleware),
        )
        .nest(
            "/guilds",
            guilds::setup_routes()
                .with(AuthenticationMiddleware)
                .with(CurrentUserMiddleware),
        )
        .nest(
            "/channels",
            channels::setup_routes()
                .with(AuthenticationMiddleware)
                .with(CurrentUserMiddleware),
        )
        .nest(
            "/invites",
            routes::invites::setup_routes()
                .with(AuthenticationMiddleware)
                .with(CurrentUserMiddleware),
        )
        .nest("/policies", routes::policies::setup_routes())
        .nest("/-", routes::health::setup_routes());

    ("/api/v9".to_string(), routes)
}
