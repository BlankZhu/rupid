use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::tcp_middleware_config::TCPMiddlewareConfig;
use crate::types;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TCPConfiguration {
    pub routers: HashMap<String, TCPRouter>,
    pub services: HashMap<String, TCPService>,
    pub middlewares: HashMap<String, TCPMiddlewareConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TCPService {
    pub load_balancer: TCPServersLoadBalancer,
    pub weighted: TCPWeightedRoudRobin,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TCPWeightedRoudRobin {
    pub services: Vec<TCPWRRService>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TCPWRRService {
    pub name: String,
    pub weight: i64,
}

impl TCPWRRService {
    pub fn set_defaults(&mut self) {
        self.weight = 1
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TCPRouter {
    pub entry_points: Vec<String>,
    pub middlewares: Vec<String>,
    pub service: Vec<String>,
    pub rule: String,
    pub tls: RouterTCPTLSConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouterTCPTLSConfig {
    pub pass_through: bool,
    pub options: String,
    pub cert_resolver: String,
    pub domains: Vec<types::domain::Domain>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TCPServersLoadBalancer {
    pub termination_delay: i64, // in milliseconds
    pub proxy_protocol: ProxyProtocol,
    pub servers: Vec<TCPServer>,
}

impl TCPServersLoadBalancer {
    pub fn mergeable(&mut self, load_balancer: Box<TCPServersLoadBalancer>) -> bool {
        // todo
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TCPServer {
    pub address: String,
    pub port: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProxyProtocol {
    pub version: i64,
}

impl ProxyProtocol {
    pub fn set_defaults(&mut self) {
        self.version = 2;
    }
}
