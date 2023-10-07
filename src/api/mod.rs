mod middleware;
mod routes;

use poem::middleware::{NormalizePath, TrailingSlash};
use poem::web::Json;
use poem::{
    listener::{Listener, TcpListener},
    post, EndpointExt, IntoResponse, Route, Server,
};
use serde_json::json;
use sqlx::MySqlPool;

use crate::api::middleware::authentication::AuthenticationMiddleware;
use crate::api::routes::{auth, channels, guilds, users};
use crate::{database, database::entities::Config, errors::Error};

pub async fn start_api(db: MySqlPool) -> Result<(), Error> {
    log::info!(target: "symfonia::api::cfg", "Loading configuration");
    let config = Config::init(&db).await?;

    if config.sentry.enabled {
        let _guard = sentry::init((
            "https://241c6fb08adb469da1bb82522b25c99f@sentry.quartzinc.space/3",
            sentry::ClientOptions {
                release: sentry::release_name!(),
                traces_sample_rate: config.sentry.trace_sample_rate as f32,
                ..Default::default()
            },
        ));
    }

    let routes = Route::new()
        .nest("/auth", auth::setup_routes())
        .nest(
            "/users",
            users::setup_routes().with(AuthenticationMiddleware),
        )
        .nest(
            "/guilds",
            guilds::setup_routes().with(AuthenticationMiddleware),
        )
        .nest(
            "/channels",
            channels::setup_routes().with(AuthenticationMiddleware),
        )
        .nest("/policies", routes::policies::setup_routes())
        .nest("/-", routes::health::setup_routes())
        .data(db)
        .data(config)
        .with(NormalizePath::new(TrailingSlash::Trim));

    let v9_api = Route::new()
        .nest("/api/v9", routes)
        .catch_all_error(custom_error);

    let bind = std::env::var("API_BIND").unwrap_or_else(|_| String::from("localhost:3001"));

    log::info!(target: "symfonia::api", "Starting HTTP Server");
    Server::new(TcpListener::bind(bind)).run(v9_api).await?;
    Ok(())
}

async fn custom_error(err: poem::Error) -> impl IntoResponse {
    Json(json! ({
        "message": err.to_string(),
    }))
}
