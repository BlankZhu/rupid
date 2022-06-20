use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Destination {
    /// FQDN or IP
    pub host: String,

    /// remote port to connection to
    pub port: u16,

    /// load balance weight
    pub weight: Option<u64>,
    // pub load_balancer: Option<LoadBalancer>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StringMatch {
    /// (ONE OF) exact match
    pub exact: Option<String>,

    /// (ONE OF) match with prefix
    pub prefix: Option<String>,

    /// (ONE OF) match with regex
    pub regex: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TLSConfig {
    /// TLS mode, default is `SIMPLE`, `MUTUAL` or `PASSTHROUGH`
    pub mode: Option<String>,

    /// root certificate content, PEM text.
    /// Only used while mode is `MUTUAL`
    pub root_cretificate: Option<String>,

    /// certificate content, PEM text
    pub certificate: Option<String>,

    /// private key content, PEM text
    pub private_key: Option<String>,

    /// minimum TLS version.
    /// TLS version includes: `TLS_AUTO`, `TLSV1_0`, `TLSV1_1`, `TLSV1_2` and `TLSV1_3`
    pub min_version: Option<String>,

    /// maximum TLS version.
    /// TLS version includes: `TLS_AUTO`, `TLSV1_0`, `TLSV1_1`, `TLSV1_2` and `TLSV1_3`
    pub max_version: Option<String>,
}
