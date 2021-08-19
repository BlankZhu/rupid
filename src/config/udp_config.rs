use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::udp_middleware_config::UDPMiddlewareConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UDPConfiguration {
    pub routers: HashMap<String, UDPRouter>,
    pub services: HashMap<String, UDPService>,
    pub middlewares: HashMap<String, UDPMiddlewareConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UDPService {
    pub load_balancer: UDPServersLoadBalancer,
    pub weighted: UDPWeightedRoundRobin,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UDPWeightedRoundRobin {
    pub services: Vec<UDPWRRService>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UDPWRRService {
    pub name: String,
    pub weight: i64,
}

impl UDPWRRService {
    pub fn set_defaults(&mut self) {
        self.weight = 1
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UDPRouter {
    pub entry_points: Vec<String>,
    pub service: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UDPServersLoadBalancer {
    pub servers: Vec<UDPServer>,
}

impl UDPServersLoadBalancer {
    pub fn Mergeable(&self, load_balancer: &UDPServersLoadBalancer) -> bool {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UDPServer {
    pub address: String,
    pub port: String,
}
