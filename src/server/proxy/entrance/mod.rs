use crate::config;
use crate::constants::*;
use hyper::http;
use log::warn;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Entrance {
    pub name: String,
    pub host: String,
    pub method: String,
    pub target: String,
}

impl Entrance {
    pub fn new(config: config::Entrance) -> Self {
        let host = config.host.map_or("".to_string(), |s| s);

        let method: String = config.method.map_or(
            RUPID_CONFIG_PROXY_ENTRANCE_DEFAULT_METHOD.to_string(),
            |method_str: String| match http::Method::from_str(method_str.as_str()) {
                Ok(_) => method_str,
                Err(e) => {
                    warn!("invalid method: {}", e);
                    RUPID_CONFIG_PROXY_BACKEND_DEFAULT_METHOD.to_string()
                }
            },
        );

        Entrance {
            name: config.name,
            host,
            method,
            target: config.target,
        }
    }
}
