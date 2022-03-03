mod proxy;

use warp::Filter;

use crate::config;

use self::proxy::Proxy;

#[derive(Debug)]
pub struct Server {
    port: u16,
    addr: [u8; 4],
    timeout: std::time::Duration,
    use_ssl: bool,
    proxies: Vec<Proxy>,
}

impl Server {
    fn new(conf: config::Server) -> Self {
        todo!()
    }

    async fn run(&self) {
        let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
        warp::serve(hello).run((self.addr, self.port)).await;
    }
}
