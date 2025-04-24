use crate::errors::Error;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
    sync::OnceLock,
};

static CONFIG: OnceLock<SymfoniaConfiguration> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize)]
pub struct SymfoniaConfiguration {
    pub mode: String,
    pub database: DatabaseConfiguration,
    pub gateway: GatewayConfiguration,
    pub api: ApiConfiguration,
}

impl SymfoniaConfiguration {
    pub fn get() -> &'static SymfoniaConfiguration {
        CONFIG.get().unwrap()
    }

    pub fn load() -> Result<(), Error> {
        let config_file = PathBuf::from("config.toml");
        let config = Self::from_file(&config_file)?;
        CONFIG.set(config);
        Ok(())
    }

    pub fn from_file(file_path: &PathBuf) -> Result<SymfoniaConfiguration, Error> {
        let file_content = std::fs::read_to_string(file_path)?;
        let config: SymfoniaConfiguration = toml::from_str(&file_content)?;

        Ok(config)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfiguration {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayConfiguration {
    pub host: String,
    pub port: u16,
}

impl Display for GatewayConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiConfiguration {
    pub host: String,
    pub port: u16,
}

impl Display for ApiConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:{}", self.host, self.port)
    }
}
