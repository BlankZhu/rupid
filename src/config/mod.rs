pub mod http_config;
pub mod http_middleware_config;
pub mod tcp_config;
pub mod tcp_middleware_config;
pub mod udp_config;
pub mod udp_middleware_config;

use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RupidConfig {
    pub http: http_config::HTTPConfiguration,
    // pub tcp: tcp_config::TCPConfiguration,
    // pub udp: udp_config::UDPConfiguration,
}

impl RupidConfig {
    pub fn load_from_yaml_file<P: AsRef<Path>>(path: P) -> Result<RupidConfig, io::Error> {
        let f = File::open(path)?;
        let rd = BufReader::new(f);
        let ret = serde_yaml::from_reader(rd)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        Ok(ret)
    }
}
