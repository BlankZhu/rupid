use std::time::{self, Duration};

use serde::{Deserialize, Serialize};

use crate::provider::Provider;

const DEFAULT_INTERNAL_ENTRYPOINT_NAME: &str = "rupid";
const DEFAULT_GRACE_TIMEOUT: u64 = 10;
const DEFAULT_IDEL_TIMEOUT: u64 = 180;
const DEFAULT_UDP_TIMEOUT: u64 = 3;
const DEFAULT_DIAL_TIMEOUT: u64 = 30;
const DEFAULT_IDEL_CONN_TIMEOUT: u64 = 180;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub servers_transport: ServersTransport,
    // pub entry_points: EntryPoints,
    pub providers: Providers,

    pub api: API,
    // pub mertrics: types::Mertrics, // todo
    // pub ping: ping::Handler, // todo

    // pub log: types::RupidLog, // todo
    // pub access_log: Types::AccessLog, // todo
    // pub tracing: Tracing,    // todo

    // pub host_resolver: types::HostResolver, // todo

    // pub certificates_resolvers: CertificatesResolvers, // todo

    // pub pilot: Pilot
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServersTransport {
    pub insecure_skip_verify: bool,
    // pub root_cas: Vec<>
    pub max_idel_conns_per_host: i64,
    pub forwarding_timeouts: ForwardingTimeouts,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct API {
    pub insecure: bool,
    pub debug: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RespondingTimeouts {
    pub read_timeout: time::Duration,
    pub write_timeout: time::Duration,
    pub idel_timeout: time::Duration,
}

impl RespondingTimeouts {
    pub fn set_defaults(&mut self) {
        self.idel_timeout = Duration::new(DEFAULT_IDEL_TIMEOUT, 0);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForwardingTimeouts {
    pub dial_timeout: time::Duration,
    pub response_header_timeout: time::Duration,
    pub idel_conn_timeout: time::Duration,
}

impl ForwardingTimeouts {
    pub fn set_defaults(&mut self) {
        self.dial_timeout = Duration::new(DEFAULT_DIAL_TIMEOUT, 0);
        self.idel_conn_timeout = Duration::new(DEFAULT_IDEL_CONN_TIMEOUT, 0);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LifeCycle {
    pub request_accept_grace_timeout: time::Duration,
    pub grace_timeout: time::Duration,
}

impl LifeCycle {
    pub fn set_defaults(&mut self) {
        self.grace_timeout = Duration::new(DEFAULT_GRACE_TIMEOUT, 0);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Providers {
    pub throttle_duration: time::Duration,
    pub file_provider: Provider,
}

impl Providers {
    pub fn optimize_configuration(&mut self) {
        // todo
    }
}
