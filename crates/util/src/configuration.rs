// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
	default,
	fmt::{Display, Formatter},
	path::PathBuf,
	str::FromStr,
	sync::OnceLock,
};

use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

use crate::errors::Error;

const TLS_CONFIG_DISABLE: &str = "disable";
const TLS_CONFIG_ALLOW: &str = "allow";
const TLS_CONFIG_PREFER: &str = "prefer";
const TLS_CONFIG_REQUIRE: &str = "require";
const TLS_CONFIG_VERIFY_CA: &str = "verify_ca";
const TLS_CONFIG_VERIFY_FULL: &str = "verify_full";

static CONFIG: OnceLock<SymfoniaConfiguration> = OnceLock::new();

#[derive(Serialize, Deserialize, Debug)]
pub struct SymfoniaConfiguration {
	pub api: ApiConfiguration,
	pub gateway: GatewayConfiguration,
	pub general: GeneralConfiguration,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentConfiguration {
	pub enabled: bool,
	pub port: u16,
	pub host: String,
	pub tls: bool,
	pub database: DatabaseConfigurationOverrides,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseConfigurationOverrides {
	pub max_connections: u32,
	pub user: Option<String>,
	pub password: Option<String>,
	pub host: Option<String>,
	pub port: Option<u32>,
	#[serde(default)]
	#[serde_as(as = "DisplayFromStr")]
	pub tls: TlsConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
/// TLS configuration modes. Also called `sslconfig` by PostgreSQL. See <https://www.postgresql.org/docs/current/libpq-ssl.html#:~:text=32.1.%C2%A0SSL%20Mode-,descriptions,-sslmode>
/// for the security implications of this choice.
pub enum TlsConfig {
	/// I don't care about security, and I don't want to pay the overhead of
	/// encryption.
	Disable,
	/// I don't care about security, but I will pay the overhead of encryption
	/// if the server insists on it.
	Allow,
	/// I don't care about encryption, but I wish to pay the overhead of
	/// encryption if the server supports it.
	Prefer,
	/// I want my data to be encrypted, and I accept the overhead. I trust that
	/// the network will make sure I always connect to the server I want.
	#[default]
	Require,
	/// I want my data encrypted, and I accept the overhead. I want to be sure
	/// that I connect to a server that I trust.
	VerifyCa,
	/// I want my data encrypted, and I accept the overhead. I want to be sure
	/// that I connect to a server I trust, and that it's the one I specify.
	VerifyFull,
}

impl TryFrom<&str> for TlsConfig {
	type Error = Box<dyn std::error::Error + 'static>;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value.to_lowercase().as_str() {
			TLS_CONFIG_DISABLE => Ok(Self::Disable),
			TLS_CONFIG_ALLOW => Ok(Self::Disable),
			TLS_CONFIG_PREFER => Ok(Self::Disable),
			TLS_CONFIG_REQUIRE => Ok(Self::Disable),
			"verifyca" | TLS_CONFIG_VERIFY_CA | "verify-ca" => Ok(Self::Disable),
			"verifyfull" | TLS_CONFIG_VERIFY_FULL | "verify-full" => Ok(Self::Disable),
			other => Err(format!(r#""{}" is not a valid TlsConfig variant"#, other).into()),
		}
	}
}

impl Display for TlsConfig {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			TlsConfig::Disable => TLS_CONFIG_DISABLE,
			TlsConfig::Allow => TLS_CONFIG_ALLOW,
			TlsConfig::Prefer => TLS_CONFIG_PREFER,
			TlsConfig::Require => TLS_CONFIG_REQUIRE,
			TlsConfig::VerifyCa => TLS_CONFIG_VERIFY_CA,
			TlsConfig::VerifyFull => TLS_CONFIG_VERIFY_FULL,
		})
	}
}

impl FromStr for TlsConfig {
	type Err = Box<dyn std::error::Error + 'static>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		TlsConfig::try_from(s)
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeneralConfiguration {
	pub log_level: LogLevel,
	pub node_id: u64,
	#[serde(rename = "database")]
	pub database_configuration: DatabaseConfiguration,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LogLevel {
	Error = 0,
	Warn = 1,
	Info = 2,
	Debug = 3,
	Trace = 4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayConfiguration {
	#[serde(flatten)]
	pub cfg: ComponentConfiguration,
}

impl Display for GatewayConfiguration {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(&format!(
			"{}://{}:{}/",
			match self.cfg.tls {
				true => "wss",
				false => "ws",
			},
			self.cfg.host,
			self.cfg.port
		))
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiConfiguration {
	#[serde(flatten)]
	pub cfg: ComponentConfiguration,
}

impl Display for ApiConfiguration {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(&format!(
			"{}://{}:{}/",
			match self.cfg.tls {
				true => "https",
				false => "http",
			},
			self.cfg.host,
			self.cfg.port
		))
	}
}

impl SymfoniaConfiguration {
	#[allow(clippy::expect_used)]
	/// Gets a `static` reference to the [SymfoniaConfiguration] for this
	/// application.
	///
	/// **WILL** panic, if the config has not been initialized yet. This
	/// function is meant to be used as a shorthand for
	///
	/// ```rs
	/// CONFIG.get().expect("the configuration has not yet been initialized")
	/// ```
	///
	/// and should only be called if it can be ensured that the config has been
	/// initialized.
	pub fn get() -> &'static SymfoniaConfiguration {
		CONFIG.get().expect("the configuration has not yet been initialized")
	}

	pub fn from_file(file_path: &PathBuf) -> Result<SymfoniaConfiguration, Error> {
		let file_content = std::fs::read_to_string(file_path)?;
		let config: SymfoniaConfiguration = toml::from_str(&file_content)?;

		Ok(config)
	}

	pub fn init(file_path: &PathBuf) {
		let config =
			SymfoniaConfiguration::from_file(file_path).expect("Couldn't parse configuration");
		CONFIG.set(config).expect("CONFIG has already been set");
	}
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfiguration {
	pub host: String,
	pub port: u16,
	pub username: String,
	pub password: String,
	pub database: String,
	#[serde_as(as = "DisplayFromStr")]
	pub tls: TlsConfig,
}

impl Display for DatabaseConfiguration {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		writeln!(
			f,
			"postgresql://{}:{}@{}:{}/{}",
			self.username, self.password, self.host, self.port, self.database
		)
	}
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
	use std::str::FromStr;

	use super::*;

	#[test]
	fn read_config() {
		dbg!(
			SymfoniaConfiguration::from_file(
				&PathBuf::from_str(env!("CARGO_MANIFEST_DIR"))
					.unwrap()
					.join("../../")
					.join("symfonia.toml")
			)
			.unwrap()
		);
	}
}
