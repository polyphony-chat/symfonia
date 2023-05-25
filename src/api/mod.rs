mod routes;

use poem::{
    listener::{Listener, TcpListener},
    post, EndpointExt, Route, Server,
};
use polyphony_types::{config, errors::Error};

use crate::{database, database::ConfigService};

pub async fn start_api() -> Result<(), Error> {
    log::info!(target: "symfonia::api::db", "Establishing database connection");
    let db = database::establish_connection().await?;

    log::info!(target: "symfonia::api::cfg", "Loading configuration");
    let config = ConfigService::init(&db).await?;

    if config.sentry.enabled {
        let _guard = sentry::init((
            "https://241c6fb08adb469da1bb82522b25c99f@sentry.quartzinc.space/3",
            sentry::ClientOptions {
                release: sentry::release_name!(),
                traces_sample_rate: config.sentry.trace_sample_rate,
                ..Default::default()
            },
        ));
    }

    let routes = Route::new()
        .nest(
            "/auth",
            Route::new().at("/login", post(routes::auth::login)),
        )
        .data(db)
        .data(config);

    let v9_api = Route::new().nest("/api/v9", routes);

    let bind = std::env::var("API_BIND").unwrap_or_else(|_| String::from("localhost:3001"));

    log::info!(target: "symfonia::api", "Starting HTTP Server");
    Server::new(TcpListener::bind(bind)).run(v9_api).await?;
    Ok(())
}
