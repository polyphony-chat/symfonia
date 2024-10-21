/*
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![allow(unused)] // TODO: Remove, I just want to clean up my build output

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use chorus::types::Snowflake;
use clap::Parser;

use gateway::{ConnectedUsers, Event};
use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        rolling_file::{
            policy::compound::{
                roll::delete::DeleteRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
    },
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    filter::Filter,
    Config,
};
use logo::print_logo;
use parking_lot::RwLock;
use pubserve::Publisher;
use tokio::sync::Mutex;

mod api;
mod cdn;
mod database;
mod errors;
mod gateway;
mod logo;
mod util;

pub type SharedEventPublisher = Arc<RwLock<Publisher<Event>>>;
pub type EventPublisherMap = HashMap<Snowflake, SharedEventPublisher>;
pub type SharedEventPublisherMap = Arc<RwLock<EventPublisherMap>>;
pub type WebSocketReceive =
    futures::stream::SplitStream<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>>;
pub type WebSocketSend = futures::stream::SplitSink<
    tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    tokio_tungstenite::tungstenite::Message,
>;

pub fn eq_shared_event_publisher(a: &SharedEventPublisher, b: &SharedEventPublisher) -> bool {
    let a = a.read();
    let b = b.read();
    *a == *b
}

// TODO: Use this in more places
/// The maximum number of rows that can be returned in most queries
static QUERY_UPPER_LIMIT: i32 = 10000;

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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "false")]
    migrate: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    dotenv::dotenv().ok();

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

    let env_mode = std::env::var("MODE").unwrap_or("DEBUG".to_string());
    let loglevel = match env_mode.to_uppercase().as_str() {
        "DEBUG" => LevelFilter::Debug,
        "PRODUCTION" => LevelFilter::Warn,
        "VERBOSE" => LevelFilter::Trace,
        _ => LevelFilter::Debug,
    };

    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(LogFilter))
                .build("stdout", Box::new(stdout)),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(LogFilter))
                .build("api", Box::new(api_log)),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(LogFilter))
                .build("cdn", Box::new(cdn_log)),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(LogFilter))
                .build("gateway", Box::new(gateway_log)),
        )
        //.logger(Logger::builder().build("symfonia::db", LevelFilter::Info))
        //.logger(Logger::builder().build("symfonia::cfg", LevelFilter::Info))
        .logger(
            Logger::builder()
                .appender("api")
                .build("symfonia::api", loglevel),
        )
        .logger(
            Logger::builder()
                .appender("cdn")
                .build("symfonia::cdn", loglevel),
        )
        .logger(
            Logger::builder()
                .appender("gateway")
                .build("symfonia::gateway", loglevel),
        )
        .build(Root::builder().appender("stdout").build(loglevel))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();

    log::info!(target: "symfonia", "Starting up Symfonia");
    print_logo();

    match loglevel {
        LevelFilter::Debug | LevelFilter::Trace => {
            log::warn!(target: "symfonia", "⚠️⚠️⚠️ WARNING ⚠️⚠️⚠️");
            log::warn!(target: "symfonia", "⚠️⚠️⚠️ WARNING ⚠️⚠️⚠️");
            log::warn!(target: "symfonia", "⚠️⚠️⚠️ WARNING ⚠️⚠️⚠️");
            log::warn!(target: "symfonia", "WARNING: Running in DEBUG or TRACE modes will leak sensitive information to the logs. Please run symfonia in production mode if you are not currently debugging. This can be done by setting the MODE environment variable to 'PRODUCTION'.");
            log::warn!(target: "symfonia", "⚠️⚠️⚠️ WARNING ⚠️⚠️⚠️");
            log::warn!(target: "symfonia", "⚠️⚠️⚠️ WARNING ⚠️⚠️⚠️");
            log::warn!(target: "symfonia", "⚠️⚠️⚠️ WARNING ⚠️⚠️⚠️");
        }
        _ => (),
    };

    log::info!(target: "symfonia::db", "Establishing database connection");
    let db = database::establish_connection()
        .await
        .expect("Failed to establish database connection");

    if database::check_migrating_from_spacebar(&db)
        .await
        .expect("Failed to check migrating from spacebar")
    {
        if !args.migrate {
            log::error!(target: "symfonia::db", "The database seems to be from spacebar.  Please run with --migrate option to migrate the database.  This is not reversible.");
            std::process::exit(0);
        } else {
            log::warn!(target: "symfonia::db", "Migrating from spacebar to symfonia");
            database::delete_spacebar_migrations(&db)
                .await
                .expect("Failed to delete spacebar migrations table");
            log::info!(target: "symfonia::db", "Running migrations");
            sqlx::migrate!("./spacebar-migrations")
                .run(&db)
                .await
                .expect("Failed to run migrations");
        }
    } else {
        sqlx::migrate!()
            .run(&db)
            .await
            .expect("Failed to run migrations");
    }

    if database::check_fresh_db(&db)
        .await
        .expect("Failed to check fresh db")
    {
        log::info!(target: "symfonia::db", "Fresh database detected.  Seeding database with config data");
        database::seed_config(&db)
            .await
            .expect("Failed to seed config");
    }

    let symfonia_config = crate::database::entities::Config::init(&db)
        .await
        .unwrap_or_default();

    let connected_users = ConnectedUsers::default();
    log::debug!(target: "symfonia", "Initializing Role->User map...");
    connected_users
        .init_role_user_map(&db)
        .await
        .expect("Failed to init role user map");
    log::trace!(target: "symfonia", "Role->User map initialized with {} entries", connected_users.role_user_map.lock().await.len());

    let mut tasks = [
        tokio::spawn(api::start_api(
            db.clone(),
            connected_users.clone(),
            symfonia_config.clone(),
        )),
        tokio::spawn(gateway::start_gateway(
            db.clone(),
            connected_users.clone(),
            symfonia_config.clone(),
        )),
    ];
    for task in tasks.iter_mut() {
        task.await
            .expect("Failed to start server")
            .expect("Failed to start server");
    }
}
