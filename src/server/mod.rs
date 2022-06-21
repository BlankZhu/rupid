pub mod http;
pub mod tcp;
pub mod tls;

use std::collections::{HashMap, HashSet};

use crate::config::server::Server as ServerConfig;
use anyhow::Result;

pub struct Server {
    name: String,
    bind_addr: String,
    port: u16,
    protocol: String,    // should be enum
    timeout: String,     // should be Timeout type
    tls: Option<String>, // should be TLS type
    https_redirect: bool,
    hosts: HashSet<String>,
    services: HashMap<String, String>, // should be Service type
}

impl Server {
    pub fn new(config: &ServerConfig) -> Server {
        todo!()
    }

    pub async fn serve(&self) -> Result<()> {
        todo!()
    }
}
