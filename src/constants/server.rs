use std::{net::Ipv4Addr, time::Duration};

pub const RUPID_DEFAULT_SERVER_BIND_ADDRESS: Ipv4Addr = Ipv4Addr::LOCALHOST;
pub const RUPID_DEFAULT_SERVER_PORT: u16 = 8000;
pub const RUPID_DEFAULT_SERVER_PROTOCOL: &str = "HTTP";
pub const RUPID_DEFAULT_READ_TIMEOUT: Duration = Duration::from_millis(5000);
pub const RUPID_DEFAULT_WRITE_TIMEOUT: Duration = Duration::from_millis(5000);
pub const RUPID_DEFAULT_HTTPS_REDIRECT: bool = false;
