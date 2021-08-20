use std::fmt;
use std::net::{AddrParseError, SocketAddr, ToSocketAddrs};
use tokio::time::Duration;

#[derive(Debug, Clone)]
pub struct CreateError {
    pub detail: String,
}

impl fmt::Display for CreateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to create proxy, detail: [{}]", self.detail)
    }
}

pub enum ProxyError {
    CreateError,
    // other errors...
}

pub struct Proxy {
    pub address: String,
    pub target: Vec<SocketAddr>,
    pub termination_delay: Duration,
    pub refresh_target: bool,
}

impl Proxy {
    pub fn new(address: &str, terminiation_delay: Duration) -> Result<Proxy, CreateError> {
        let mut refresh = false;
        let mut target = Vec::new();

        // check if address is [IP:PORT] or [HOST:PORT]
        let parse_res: Result<SocketAddr, AddrParseError> = address.parse();
        match parse_res {
            Ok(addr) => {
                target.push(addr);
            }
            Err(_) => {
                // check if is [HOST:PORT]
                target = address
                    .to_socket_addrs()
                    .map_err(|err| CreateError {
                        detail: err.to_string(),
                    })?
                    .collect();
                refresh = true;
            }
        }

        Ok(Proxy {
            address: address.to_string(),
            target,
            termination_delay: terminiation_delay,
            refresh_target: refresh,
        })
    }

    pub fn server_tcp() {
        
    }
}
