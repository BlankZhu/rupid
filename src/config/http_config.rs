use std::{collections::HashMap, time};

use crate::types;

use super::http_middleware_config::HTTPMiddlewareConfig;

pub struct HTTPConfiguration {
    pub routers: HashMap<String, Box<Router>>,
    pub services: HashMap<String, Box<Service>>,
    pub middlewares: HashMap<String, Box<HTTPMiddlewareConfig>>,
    pub models: HashMap<String, Box<Model>>,
    pub servers_transports: HashMap<String, Box<ServersTransport>>,
}

pub struct Model {
    pub middlewares: Vec<String>,
    pub tls: Box<RouterTSLConfig>,
}

pub struct Service {
    pub load_balancer: Box<ServersLoadBalancer>,
    pub weighted: Box<WeightedRoundRobin>,
    pub mirroring: Box<Mirroring>,
}

pub struct Router {
    pub entry_points: Vec<String>,
    pub middlewares: Vec<String>,
    pub service: String,
    pub priority: i64,
    pub tls: Box<RouterTSLConfig>,
}

pub struct RouterTSLConfig {
    pub options: String,
    pub cert_resolver: String,
    pub domains: Vec<types::domain::Domain>,
}

pub struct Mirroring {
    pub service: String,
    pub max_body_size: i64,
    pub mirrors: MirrorService,
    pub health_check: Box<HealthCheck>,
}

impl Mirroring {
    pub fn set_defaults(&mut self) {
        self.max_body_size = -1;
    }
}

pub struct MirrorService {
    pub name: String,
    pub precent: i64,
}

pub struct WeightedRoundRobin {
    pub services: Vec<WRService>,
    pub sticky: Box<Sticky>,
    pub health_check: Box<HealthCheck>,
}

pub struct WRService {
    pub name: String,
    pub weight: i64,
}

impl WRService {
    pub fn set_defaults(&mut self) {
        self.weight = 1
    }
}

pub struct Sticky {
    pub cookie: Box<Cookie>,
}

pub struct Cookie {
    pub name: String,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: String,
}

pub struct ServersLoadBalancer {
    pub sticky: Box<Sticky>,
    pub servers: Vec<Server>,
    pub health_check: Box<HealthCheck>,
    pub pass_host_header: bool,
    pub response_forwarding: Box<ResponseForwarding>,
    pub servvers_transport: String,
}

impl ServersLoadBalancer {
    // maybe should use a trait
    pub fn mergeable(&self, load_balancer: Box<ServersLoadBalancer>) -> bool {
        // todo
        todo!()
    }

    pub fn set_defaults(&mut self) {
        self.pass_host_header = true
    }
}

pub struct ResponseForwarding {
    pub flush_internal: String,
}

pub struct Server {
    pub url: String,
    pub scheme: String,
    pub port: String,
}

impl Server {
    pub fn set_defaults(&mut self) {
        self.scheme = "http".to_string();
    }
}

pub struct ServerHealthCheck {
    pub scheme: String,
    pub path: String,
    pub port: i64,
    pub interval: time::Duration,
    pub timeout: time::Duration,
    pub follow_redirects: bool,
    pub headers: HashMap<String, String>,
}

impl ServerHealthCheck {
    pub fn set_defaults(&mut self) {
        self.follow_redirects = true
    }
}

pub struct HealthCheck {}

pub struct ServersTransport {
    pub server_name: String,
    pub insecure_skip_verify: bool,
    // pub root_cas: Vec<>  // todo: add cerificates type
    pub max_idle_conns_per_host: i64,
    pub forwarding_timeout: ForwardingTimeout,
    pub disable_http2: bool,
    pub peer_cert_uri: String,
}

pub struct ForwardingTimeout {
    pub dial_timeout: time::Duration,
    pub response_header_timeout: time::Duration,
    pub idel_conn_timeout: time::Duration,
}
