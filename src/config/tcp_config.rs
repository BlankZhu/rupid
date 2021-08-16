use std::collections::HashMap;

use crate::types;

use super::tcp_middleware_config::TCPMiddlewareConfig;

pub struct TCPConfiguration {
    pub routers: HashMap<String, TCPRouter>,
    pub services: HashMap<String, TCPService>,
    pub middlewares: HashMap<String, TCPMiddlewareConfig>,
}

pub struct TCPService {
    pub load_balancer: Box<TCPServersLoadBalancer>,
    pub weighted: Box<TCPWeightedRoudRobin>,
}

pub struct TCPWeightedRoudRobin {
    pub services: Vec<TCPWRRService>,
}

pub struct TCPWRRService {
    pub name: String,
    pub weight: i64,
}

impl TCPWRRService {
    pub fn set_defaults(&mut self) {
        self.weight = 1
    }
}

pub struct TCPRouter {
    pub entry_points: Vec<String>,
    pub middlewares: Vec<String>,
    pub service: Vec<String>,
    pub rule: String,
    pub tls: Box<RouterTCPTLSConfig>,
}

pub struct RouterTCPTLSConfig {
    pub pass_through: bool,
    pub options: String,
    pub cert_resolver: String,
    pub domains: Vec<types::domain::Domain>,
}

pub struct TCPServersLoadBalancer {
    pub termination_delay: i64, // in milliseconds
    pub proxy_protocol: Box<ProxyProtocol>,
    pub servers: Vec<TCPServer>,
}

impl TCPServersLoadBalancer {
    pub fn mergeable(&mut self, load_balancer: Box<TCPServersLoadBalancer>) -> bool {
        // todo
        todo!()
    }
}

pub struct TCPServer {
    pub address: String,
    pub port: String,
}

pub struct ProxyProtocol {
    pub version: i64,
}

impl ProxyProtocol {
    pub fn set_defaults(&mut self) {
        self.version = 2;
    }
}
