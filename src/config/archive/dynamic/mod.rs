pub mod certificate;
pub mod http_config;
pub mod http_middleware_config;
pub mod tcp_config;
pub mod tcp_middleware_config;
pub mod tls_config;
pub mod udp_config;
pub mod udp_middleware_config;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub provider_name: String,
    pub rupid_config: RupidConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RupidConfig {
    pub http: http_config::HTTPConfiguration,
    pub tcp: tcp_config::TCPConfiguration,
    pub udp: udp_config::UDPConfiguration,
    // pub tls: tls_config::TLSConfiguration, // todo
}
