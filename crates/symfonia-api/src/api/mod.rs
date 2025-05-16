// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use poem::{
	EndpointExt, IntoResponse, Route, Server,
	listener::TcpListener,
	middleware::{Cors, NormalizePath, TrailingSlash},
	web::Json,
};
use reqwest::Method;
use serde_json::json;
use sqlx::PgPool;
use util::{
	configuration::SymfoniaConfiguration, entities::Config, errors::Error, gateway::ConnectedUsers,
};

use crate::api::{
	middleware::{authentication::AuthenticationMiddleware, current_user::CurrentUserMiddleware},
	routes::{auth, channels, guilds, users},
};

mod middleware;
mod routes;

pub async fn start_api(
	db: PgPool,
	connected_users: ConnectedUsers,
	config: Config,
) -> Result<(), Error> {
	log::info!(target: "symfonia::api::cfg", "Loading configuration");

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

	let v9_api = Route::new()
		.at("/ping", routes::ping::setup_routes())
		.at("/version", routes::version::setup_routes())
		.nest("/api", setup_api_routes())
		.nest("/api/v9", setup_api_routes())
		.data(db)
		.data(config)
		.data(connected_users)
		.with(NormalizePath::new(TrailingSlash::Trim))
		.with(Cors::new().allow_methods(&[
			Method::CONNECT,
			Method::DELETE,
			Method::GET,
			Method::HEAD,
			Method::OPTIONS,
			Method::PATCH,
			Method::POST,
			Method::PUT,
			Method::TRACE,
		]))
		.catch_all_error(custom_error);

	log::info!(target: "symfonia::api", "Starting HTTP Server");

	let host = &SymfoniaConfiguration::get().api.cfg.host;
	let port = SymfoniaConfiguration::get().api.cfg.port;

	tokio::task::spawn(async move {
		// .trim() needs to be called because \n is appended to the .to_string(),
		// messing up the binding
		Server::new(TcpListener::bind((host.as_str(), port)))
			.run(v9_api)
			.await
			.expect("Failed to start HTTP server");
		log::info!(target: "symfonia::api", "HTTP Server stopped");
	});

	log::info!(target: "symfonia::api", "HTTP Server listening on {}", SymfoniaConfiguration::get().api);
	Ok(())
}

fn setup_api_routes() -> Route {
	Route::new()
		.nest("/auth", auth::setup_routes())
		.nest(
			"/users",
			users::setup_routes().with(AuthenticationMiddleware).with(CurrentUserMiddleware),
		)
		.nest(
			"/guilds",
			guilds::setup_routes().with(AuthenticationMiddleware).with(CurrentUserMiddleware),
		)
		.nest(
			"/channels",
			channels::setup_routes().with(AuthenticationMiddleware).with(CurrentUserMiddleware),
		)
		.nest(
			"/invites",
			routes::invites::setup_routes()
				.with(AuthenticationMiddleware)
				.with(CurrentUserMiddleware),
		)
		.nest("/policies", routes::policies::setup_routes())
		.nest("/-", routes::health::setup_routes())
		.at("/version", routes::version::setup_routes())
		.at("/ping", routes::ping::setup_routes())
}

async fn custom_error(err: poem::Error) -> impl IntoResponse {
	Json(json! ({
		"success": false,
		"message": err.to_string(),
	}))
	.with_status(err.status())
}
