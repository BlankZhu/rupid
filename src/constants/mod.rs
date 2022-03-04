pub const RUPID_CONFIG_SERVER_DEFAULT_BIND_ADDRESS: &str = "127.0.0.1";
pub const RUPID_CONFIG_SERVER_DEFAULT_PORT: u16 = 7777;
pub const RUPID_CONFIG_SERVER_DEFAULT_TIMEOUT: std::time::Duration =
    std::time::Duration::from_millis(7000);
pub const RUPID_CONFIG_SERVER_DEFAULT_USE_SSL: bool = false;

// proxy config related
pub const RUPID_CONFIG_PROXY_DEFAULT_LOAD_BALANCER: &str = "roundrobin";

// log config realted
pub const RUPID_CONFIG_PROXY_LOG_DEFAULT_TO_STD_OUT: bool = true;

// entrace config related
pub const RUPID_CONFIG_PROXY_ENTRANCE_DEFAULT_METHOD: &str = "ALL";

// middleware config related
// todo...

// load balancer config related
pub const RUPID_CONFIG_PROXY_LOAD_BALANCER_ROUND_ROBIN: &str = "roundrobin";
// todo...

// backend config related
pub const RUPID_CONFIG_PROXY_BACKEND_DEFAULT_PORT: u16 = 80;
pub const RUPID_CONFIG_PROXY_BACKEND_DEFAULT_USE_SSL: bool = false;
pub const RUPID_CONFIG_PROXY_BACKEND_DEFAULT_METHOD: &str = "PRESERVE";
pub const RUPID_CONFIG_PROXY_BACKEND_DEFAULT_TIMEOUT: std::time::Duration =
    std::time::Duration::from_millis(5000);
