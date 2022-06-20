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
