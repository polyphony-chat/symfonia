use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use lazy_static::lazy_static;
use log::trace;

mod cli;

lazy_static! {
	static ref CLI_ARGS: cli::CliArgs = cli::CliArgs::try_parse().unwrap_or_default();
}

pub(crate) type AnyError = Box<dyn std::error::Error + 'static>;

#[tokio::main]
async fn main() -> Result<(), AnyError> {
	env_logger::builder()
		.filter(None, log::LevelFilter::Off)
		.filter_module("symfonia", log::LevelFilter::Trace)
		.try_init()?;
	let _ = cli::CliArgs::try_parse().unwrap_or_default();
	let config: util::configuration::SymfoniaConfiguration =
		toml::from_str(&std::fs::read_to_string(
			CLI_ARGS.config.as_ref().unwrap_or(
				&PathBuf::from_str("./symfonia.toml")
					.expect("This str is not a valid PathBuf: Report this to the developers"),
			),
		)?)?;
	trace!("Read config!");
	Ok(())
}
