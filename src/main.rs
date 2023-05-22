#![recursion_limit = "256"]

use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    filter::Filter,
    Config,
};

mod api;
mod cdn;
mod database;
mod errors;
mod gateway;
mod util;

#[derive(Debug)]
struct LogFilter;

impl Filter for LogFilter {
    fn filter(&self, record: &log::Record) -> log4rs::filter::Response {
        if record.target().starts_with("spacebar") {
            log4rs::filter::Response::Accept
        } else {
            log4rs::filter::Response::Reject
        }
    }
}

fn main() {
    dotenv::dotenv().ok();

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("")))
        .build();

    let api_log = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("log/api.log")
        .unwrap();

    let cdn_log = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("log/cdn.log")
        .unwrap();

    let gateway_log = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("log/gateway.log")
        .unwrap();

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
        .logger(Logger::builder().build("spacebar::db", LevelFilter::Info))
        .logger(Logger::builder().build("spacebar::cfg", LevelFilter::Info))
        .logger(
            Logger::builder()
                .appender("api")
                .additive(false)
                .build("spacebar::api", LevelFilter::Warn),
        )
        .logger(
            Logger::builder()
                .appender("cdn")
                .additive(false)
                .build("spacebar::cdn", LevelFilter::Warn),
        )
        .logger(
            Logger::builder()
                .appender("gateway")
                .additive(false)
                .build("spacebar::gateway", LevelFilter::Warn),
        )
        .build(Root::builder().appender("stdout").build({
            let mode = std::env::var("MODE").unwrap_or("DEBUG".to_string());
            match mode.as_str() {
                "DEBUG" => LevelFilter::Debug,
                "PRODUCTION" => LevelFilter::Warn,
                _ => LevelFilter::Trace,
            }
        }))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();

    log::info!(target: "spacebar", "Starting up Oxidized Spacebar.chat");
}
