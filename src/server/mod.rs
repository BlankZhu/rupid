mod proxy;

use crate::config;
use crate::constants::*;

use self::proxy::Proxy;

use axum::Router;
use hyper::Body;
use log::error;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Server {
    port: u16,
    addr: SocketAddr,
    timeout: Duration,
    use_ssl: bool,
    proxies: Vec<Proxy>,
}

impl Server {
    pub fn new(config: config::Server) -> Self {
        let port = config.port.map_or(RUPID_CONFIG_SERVER_DEFAULT_PORT, |p| p);

        let default_addr = RUPID_CONFIG_SERVER_DEFAULT_BIND_ADDRESS
            .to_string()
            .parse::<IpAddr>()
            .unwrap();
        let addr = config.address.map_or(default_addr, |str: String| {
            str.parse::<IpAddr>().map_or(default_addr, |a| a)
        });
        let socket_addr = SocketAddr::from((addr, port));

        let timeout = config
            .timeout
            .map_or(RUPID_CONFIG_SERVER_DEFAULT_TIMEOUT, |timeout: u64| {
                Duration::from_millis(timeout)
            });

        let use_ssl = config
            .use_ssl
            .map_or(RUPID_CONFIG_SERVER_DEFAULT_USE_SSL, |b| b);

        let mut proxies = Vec::new();
        for p in config.proxies {
            let proxy = Proxy::new(p);
            proxies.push(proxy);
        }

        Server {
            port,
            addr: socket_addr,
            timeout,
            use_ssl,
            proxies,
        }
    }

    pub async fn run(&self) {
        let mut server: Router<Body> = Router::new();
        for p in self.proxies.iter() {
            let router = p.generate_router();
            server = server.merge(router);
        }

        let make_service = server.into_make_service();
        let hyper_server = hyper::Server::bind(&self.addr).serve(make_service);
        match hyper_server.await {
            Ok(_) => return,
            Err(e) => {
                error!("server [NAME] error, detail: {}", e);
                return;
            }
        }
    }
}
