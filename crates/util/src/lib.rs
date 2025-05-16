// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(unused)] // TODO: Remove, I just want to clean up my build output

use std::{
	collections::{HashMap, HashSet},
	sync::Arc,
};

use chorus::types::Snowflake;
use log::LevelFilter;
use log4rs::{
	Config,
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
use parking_lot::RwLock;
use pubserve::Publisher;
use sqlx::PgPool;
use tokio::sync::{Mutex, OnceCell};

use crate::{configuration::SymfoniaConfiguration, gateway::event::Event};

pub mod configuration;
pub mod database;
pub mod entities;
pub mod errors;
pub mod events;
pub mod gateway;
pub mod util;

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

#[cfg(test)]
#[allow(clippy::expect_used)]
/// Publicly exported function so that all symfonia-* tests can run with
/// logging. This function is only exported if `#[cfg(test)]` is met.
pub fn init_logger() {
	env_logger::builder()
		.filter_level(LevelFilter::Off)
		.filter_module("symfonia", LevelFilter::Trace)
		.filter_module("util", LevelFilter::Trace)
		.filter_module("symfonia-api", LevelFilter::Trace)
		.filter_module("symfonia-gateway", LevelFilter::Trace)
		.try_init()
		.expect("Failed initializing logger");
}
