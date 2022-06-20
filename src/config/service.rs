use std::collections::HashMap;

use super::common::{Destination, StringMatch, TLSConfig};
use serde::{Deserialize, Serialize};

use super::common::Destination as TCPService;

/// describes the possible upstream services
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Service {
    /// name of the service
    pub name: String,

    /// host the service match (only for TLS/HTTP/HTTPS)
    pub hosts: Vec<String>,

    /// (Protocol; ONE OF) local dir as service, used for web server
    pub local: Option<LocalService>,

    /// (Protocol; ONE OF) TCP service, used as TCP proxy
    pub tcp: Option<Vec<TCPService>>,

    /// (Protocol; ONE OF) TLS service, used as TLS proxy
    pub tls: Option<Vec<TLSService>>,

    /// (Protocol; ONE OF) HTTP service, used as HTTP proxy
    pub http: Option<Vec<HTTPService>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocalService {
    /// path to content dir
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TLSService {
    /// matching SNI host
    pub r#match: Vec<String>,

    /// TLS upstream destination
    pub route: Vec<Destination>,

    /// tls related config
    pub config: TLSConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HTTPService {
    /// HTTP matching conditions, linked by `AND`
    pub r#match: Vec<HTTPMatch>,

    /// HTTP upstream destination
    pub route: Vec<Destination>,

    /// tls related config
    pub config: Option<TLSConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HTTPMatch {
    /// URI matching condition
    pub uri: Option<StringMatch>,

    /// scheme matching condition
    pub scheme: Option<StringMatch>,

    /// HTTP method matching condition
    pub method: Option<StringMatch>,

    /// authority matching condition
    pub authority: Option<StringMatch>,

    /// header matching condition
    pub headers: Option<HashMap<String, StringMatch>>,

    /// URI query param matching condition
    pub queries: Option<HashMap<String, StringMatch>>,

    /// ignore cases in HTTPMatch, default false
    pub ignore_case: Option<bool>,
}
