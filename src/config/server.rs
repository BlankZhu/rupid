use serde::{Deserialize, Serialize};

/// describes the server listening to incoming connections.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Server {
    /// server's name, must be unique
    pub name: String,

    /// address to bind, default `127.0.0.1`
    pub bind_address: Option<String>,

    /// port to listen to the connection
    pub port: u16,

    /// which protocol does the server handle, including `TCP`, `TLS`, `HTTP`, `HTTPS`
    pub protocol: String,

    /// connection I/O timeout, in ms, default 5s for I/O
    pub timeout: Option<Timeout>,

    /// TLS config, default empty
    pub tls: Option<TLS>,

    /// redirect HTTP connection to HTTPS by returing a 301 response.
    /// Default is false.
    https_redirect: Option<bool>,

    /// hosts the server accepts
    pub hosts: Vec<String>,

    /// upstream services the server attaches to
    pub services: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Timeout {
    /// connection read timeout, default is 5s
    pub read_timeout: Option<u128>,

    /// connection write timeout, default is 5s
    pub write_timeout: Option<u128>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TLS {
    /// TLS mode, default is `SIMPLE`, or `MUTUAL`
    pub mode: Option<String>,

    /// private key content, PEM text
    pub private_key: String,

    /// certificate content, PEM text
    pub certificate: String,

    /// root certificate content, PEM text.
    /// Only used while mode is `MUTUAL`
    pub root_certificate: Option<String>,

    /// minimum TLS version.
    /// TLS version includes: `TLS_AUTO`, `TLSV1_0`, `TLSV1_1`, `TLSV1_2` and `TLSV1_3`
    min_version: Option<String>,

    /// maximum TLS version.
    /// TLS version includes: `TLS_AUTO`, `TLSV1_0`, `TLSV1_1`, `TLSV1_2` and `TLSV1_3`
    max_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HTTP {
    pub min_version: Option<String>,
    pub max_version: Option<String>,
    // pub keep_alive: Option<>
}
