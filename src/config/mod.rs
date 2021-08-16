pub mod certificate;
pub mod http_config;
pub mod http_middleware_config;
pub mod tcp_config;
pub mod tcp_middleware_config;
pub mod tls_config;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RupidConfig {

}

impl RupidConfig {
    
}
