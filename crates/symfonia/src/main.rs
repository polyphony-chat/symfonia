// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use lazy_static::lazy_static;
use log::{LevelFilter, trace};
use log4rs::{
	append::{
		console::{ConsoleAppender, Target},
		rolling_file::{
			RollingFileAppender,
			policy::compound::{
				CompoundPolicy, roll::delete::DeleteRoller, trigger::size::SizeTrigger,
			},
		},
	},
	config::{Appender, Logger, Root},
	encode::pattern::PatternEncoder,
	filter::Filter,
};
use sqlx::{PgPool, postgres::PgConnectOptions};
use symfonia_api::api::start_api;
use symfonia_gateway::start_gateway;
use tokio::sync::OnceCell;
use util::{
	configuration::SymfoniaConfiguration, database::Connection, entities::Config,
	gateway::ConnectedUsers,
};

mod cli;

lazy_static! {
	static ref CLI_ARGS: cli::CliArgs = cli::CliArgs::try_parse().unwrap_or_default();
}
static DATABASE: OnceCell<PgPool> = OnceCell::const_new();

#[derive(Debug)]
struct LogFilter;

impl Filter for LogFilter {
	fn filter(&self, record: &log::Record) -> log4rs::filter::Response {
		if record.target().starts_with("symfonia") {
			log4rs::filter::Response::Accept
		} else {
			log4rs::filter::Response::Reject
		}
	}
}

pub(crate) type AnyError = Box<dyn std::error::Error + 'static>;

#[tokio::main]
async fn main() -> Result<(), AnyError> {
	let _ = cli::CliArgs::try_parse().unwrap_or_default();
	SymfoniaConfiguration::init(
		CLI_ARGS.config.as_ref().unwrap_or(
			&PathBuf::from_str("./symfonia.toml")
				.expect("This str is not a valid PathBuf: Report this to the developers"),
		),
	);
	trace!("Read config!");
	let stdout = ConsoleAppender::builder()
		.target(Target::Stdout)
		.encoder(Box::new(PatternEncoder::new(
			"{d(%Y-%m-%d %H:%M:%S)} | {h({l:<6.6})} | {t:<35} | {m}{n}",
		)))
		.build();

	let api_log = RollingFileAppender::builder()
		.encoder(Box::new(PatternEncoder::new("{d} {l} - {m}{n}")))
		.build(
			"log/api.log",
			Box::new(CompoundPolicy::new(
				Box::new(SizeTrigger::new(1024 * 1024 * 30)),
				Box::new(DeleteRoller::new()),
			)),
		)
		.unwrap();

	let cdn_log = RollingFileAppender::builder()
		.encoder(Box::new(PatternEncoder::new("{d} {l} - {m}{n}")))
		.build(
			"log/cdn.log",
			Box::new(CompoundPolicy::new(
				Box::new(SizeTrigger::new(1024 * 1024 * 30)),
				Box::new(DeleteRoller::new()),
			)),
		)
		.unwrap();

	let gateway_log = RollingFileAppender::builder()
		.encoder(Box::new(PatternEncoder::new("{d} {l} - {m}{n}")))
		.build(
			"log/gateway.log",
			Box::new(CompoundPolicy::new(
				Box::new(SizeTrigger::new(1024 * 1024 * 30)),
				Box::new(DeleteRoller::new()),
			)),
		)
		.unwrap();

	// let loglevel = match
	// SymfoniaConfiguration::get().mode.to_uppercase().as_str() { 	"DEBUG" =>
	// LevelFilter::Debug, 	"PRODUCTION" => LevelFilter::Warn,
	// 	"VERBOSE" => LevelFilter::Trace,
	// 	_ => LevelFilter::Debug,
	// };

	let loglevel = LevelFilter::Trace;

	let config = log4rs::Config::builder()
		.appender(Appender::builder().filter(Box::new(LogFilter)).build("stdout", Box::new(stdout)))
		.appender(Appender::builder().filter(Box::new(LogFilter)).build("api", Box::new(api_log)))
		.appender(Appender::builder().filter(Box::new(LogFilter)).build("cdn", Box::new(cdn_log)))
		.appender(
			Appender::builder().filter(Box::new(LogFilter)).build("gateway", Box::new(gateway_log)),
		)
		//.logger(Logger::builder().build("symfonia::db", LevelFilter::Info))
		//.logger(Logger::builder().build("symfonia::cfg", LevelFilter::Info))
		.logger(Logger::builder().appender("api").build("symfonia::api", loglevel))
		.logger(Logger::builder().appender("cdn").build("symfonia::cdn", loglevel))
		.logger(Logger::builder().appender("gateway").build("symfonia::gateway", loglevel))
		.build(Root::builder().appender("stdout").build(loglevel))
		.unwrap();

	let _handle = log4rs::init_config(config).unwrap();

	log::info!(target: "symfonia", "Starting up Symfonia");
	// print_logo();

	match loglevel {
		LevelFilter::Debug | LevelFilter::Trace => {
			log::warn!(target: "symfonia", "⚠️⚠️⚠️ WARNING ⚠️⚠️⚠️");
			log::warn!(target: "symfonia", "⚠️⚠️⚠️ WARNING ⚠️⚠️⚠️");
			log::warn!(target: "symfonia", "⚠️⚠️⚠️ WARNING ⚠️⚠️⚠️");
			log::warn!(target: "symfonia", r#"WARNING: Running in "DEBUG" or "VERBOSE" modes will leak sensitive information to the logs. Please run symfonia in production mode if you are not currently debugging. This can be done by setting the `mode` option in your config to "PRODUCTION"."#);
			log::warn!(target: "symfonia", "⚠️⚠️⚠️ WARNING ⚠️⚠️⚠️");
			log::warn!(target: "symfonia", "⚠️⚠️⚠️ WARNING ⚠️⚠️⚠️");
			log::warn!(target: "symfonia", "⚠️⚠️⚠️ WARNING ⚠️⚠️⚠️");
		}
		_ => (),
	};

	let pg_connect_options = PgConnectOptions::new()
		.database(&SymfoniaConfiguration::get().general.database_configuration.database)
		.application_name("symfonia-monolith")
		.host(&SymfoniaConfiguration::get().general.database_configuration.host)
		.password(&SymfoniaConfiguration::get().general.database_configuration.password)
		.port(SymfoniaConfiguration::get().general.database_configuration.port)
		.ssl_mode(match &SymfoniaConfiguration::get().general.database_configuration.tls {
			util::configuration::TlsConfig::Disable => sqlx::postgres::PgSslMode::Disable,
			util::configuration::TlsConfig::Allow => sqlx::postgres::PgSslMode::Allow,
			util::configuration::TlsConfig::Prefer => sqlx::postgres::PgSslMode::Prefer,
			util::configuration::TlsConfig::Require => sqlx::postgres::PgSslMode::Require,
			util::configuration::TlsConfig::VerifyCa => sqlx::postgres::PgSslMode::VerifyCa,
			util::configuration::TlsConfig::VerifyFull => sqlx::postgres::PgSslMode::VerifyFull,
		})
		.username(&SymfoniaConfiguration::get().general.database_configuration.username);
	log::info!(target: "symfonia::db", "Establishing database connection");
	let db = Connection::new(pg_connect_options).await?;

	// if database::check_migrating_from_spacebar(db)
	// 	.await
	// 	.expect("Failed to check migrating from spacebar")
	// {
	// 	if !args.migrate {
	// 		log::error!(target: "symfonia::db", "The database seems to be from
	// spacebar.  Please run with --migrate option to migrate the database.  This is
	// not reversible."); 		std::process::exit(0);
	// 	} else {
	// 		log::warn!(target: "symfonia::db", "Migrating from spacebar to symfonia");
	// 		database::delete_spacebar_migrations(db)
	// 			.await
	// 			.expect("Failed to delete spacebar migrations table");
	// 		log::info!(target: "symfonia::db", "Running migrations");
	// 		sqlx::migrate!("./spacebar-migrations")
	// 			.run(db)
	// 			.await
	// 			.expect("Failed to run migrations");
	// 	}
	// } else
	{
		sqlx::migrate!("../util/migrations")
			.run(db.pool())
			.await
			.expect("Failed to run migrations");
	}

	if db.check_fresh_db().await.expect("Failed to check fresh db") {
		log::info!(target: "symfonia::db", "Fresh database detected.  Seeding database with config data");
		db.seed_config().await.expect("Failed to seed config");
	}

	let symfonia_config = Config::init(db.pool()).await.unwrap_or_default();

	let connected_users = ConnectedUsers::default();
	log::debug!(target: "symfonia", "Initializing Role->User map...");
	connected_users.init_role_user_map(db.pool()).await.expect("Failed to init role user map");
	log::trace!(target: "symfonia", "Role->User map initialized with {} entries", connected_users.role_user_map.lock().await.len());

	let mut tasks = [
		tokio::spawn(start_api(
			db.pool().to_owned(),
			connected_users.clone(),
			symfonia_config.clone(),
		)),
		tokio::spawn(start_gateway(
			db.pool().to_owned(),
			connected_users.clone(),
			symfonia_config.clone(),
		)),
	];
	for task in tasks.iter_mut() {
		task.await.expect("Failed to start server").expect("Failed to start server");
	}
	Ok(())
}
