use std::path::PathBuf;

#[derive(Debug, clap::Parser, Default)]
pub(crate) struct CliArgs {
	#[arg(short, long)]
	/// Path to the symfonia `TOML` configuration file. Will assume
	/// "./symfonia.toml", if not specified.
	pub(crate) config: Option<PathBuf>,
}
