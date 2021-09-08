use std::{collections::HashMap, time};

use serde::{Deserialize, Serialize};

use super::http_middleware_config::HTTPMiddlewareConfig;
use crate::types;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HTTPConfiguration {
    pub routers: HashMap<String, Router>,   // path to Router
    pub services: HashMap<String, Service>, // upstream service name to Service
    pub middlewares: HashMap<String, HTTPMiddlewareConfig>, // middleware name to Middleware
    pub models: HashMap<String, Model>, // 
    pub servers_transports: HashMap<String, ServersTransport>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    pub middlewares: Vec<String>,
    pub tls: RouterTSLConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Service {
    pub load_balancer: ServersLoadBalancer,
    pub weighted: WeightedRoundRobin,
    pub mirroring: Mirroring,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Router {
    pub entry_points: Vec<String>,
    pub middlewares: Vec<String>,
    pub service: String,
    pub priority: i64,
    pub tls: RouterTSLConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouterTSLConfig {
    pub options: String,
    pub cert_resolver: String,
    pub domains: Vec<types::domain::Domain>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mirroring {
    pub service: String,
    pub max_body_size: i64,
    pub mirrors: MirrorService,
    pub health_check: HealthCheck,
}

impl Mirroring {
    pub fn set_defaults(&mut self) {
        self.max_body_size = -1;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MirrorService {
    pub name: String,
    pub precent: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeightedRoundRobin {
    pub services: WRService,
    pub sticky: Sticky,
    pub health_check: HealthCheck,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WRService {
    pub name: String,
    pub weight: i64,
}

impl WRService {
    pub fn set_defaults(&mut self) {
        self.weight = 1
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sticky {
    pub cookie: Cookie,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServersLoadBalancer {
    pub sticky: Sticky,
    pub servers: Vec<Server>,
    pub health_check: HealthCheck,
    pub pass_host_header: bool,
    pub response_forwarding: ResponseForwarding,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseForwarding {
    pub flush_internal: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealthCheck {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServersTransport {
    pub server_name: String,
    pub insecure_skip_verify: bool,
    // pub root_cas: Vec<>  // todo: add cerificates type
    pub max_idle_conns_per_host: i64,
    pub forwarding_timeout: ForwardingTimeout,
    pub disable_http2: bool,
    pub peer_cert_uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForwardingTimeout {
    pub dial_timeout: time::Duration,
    pub response_header_timeout: time::Duration,
    pub idel_conn_timeout: time::Duration,
}
