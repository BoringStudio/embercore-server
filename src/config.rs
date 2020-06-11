use std::net::SocketAddr;

use config::{
    Config as RowConfig,
    ConfigError,
    File,
    FileFormat,
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server_address:           SocketAddr,
    pub redis_url:                String,
    pub database_url:             String,
    pub database_user_name:       String,
    pub database_user_password:   String,
    pub database_max_connections: u32,
}

impl Config {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        let mut config = RowConfig::new();
        config.merge(File::new(path, FileFormat::Yaml))?;
        config.try_into()
    }
}
