pub mod common;
pub mod log;
pub mod middleware;
pub mod server;
pub mod service;

use serde::{Deserialize, Serialize};

/// describes the Rupid config
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub log: log::Log,
    pub middlewares: middleware::Middlewares,
    pub servers: Vec<server::Server>,
    pub services: Vec<service::Service>,
}

impl Config {
    pub fn read_from_file(filename: &str) -> anyhow::Result<Config> {
        let config_file = std::fs::File::open(filename)?;
        let rupid_config: Config = serde_yaml::from_reader(config_file)?;
        Ok(rupid_config)
    }
}
