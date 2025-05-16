// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::PathBuf;

#[derive(Debug, clap::Parser, Default)]
pub(crate) struct CliArgs {
	#[arg(short, long)]
	/// Path to the symfonia `TOML` configuration file. Will assume
	/// "./symfonia.toml", if not specified.
	pub(crate) config: Option<PathBuf>,
}
