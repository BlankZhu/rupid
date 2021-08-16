use std::collections::HashMap;

use super::certificate::Certificate;

pub struct TLSConfiguration {
    pub certificates: Vec<Box<CertAndStores>>,
    pub options: HashMap<String, TLSOption>,
    pub stores: HashMap<String, TLSStore>,
}

pub struct ClientAuth {
    pub ca_files: Vec<String>,
    pub client_auth_type: String,
}

pub struct TLSOption {
    pub min_version: String,
    pub max_version: String,
    pub cipher_suites: Vec<String>,
    // pub curve_preferences: Vec<String>,
    pub client_auth: ClientAuth,
    pub sni_strict: bool,
    pub prefer_server_cipher_suites: bool,
}

pub struct TLSStore {
    pub default_certificate: Box<Certificate>,
}

pub struct CertAndStores {
    pub certificate: Certificate,
    pub stores: Vec<String>,
}
