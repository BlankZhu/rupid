const RUPID_CONFIG_DEFAULT_BIND_ADDRESS: [i32; 4] = [127, 0, 0, 1];
const RUPID_CONFIG_DEFAULT_PORT: u16 = 7777;
const RUPID_CONFIG_DEFAULT_USE_SSL: bool = false;

// proxy config related
const RUPID_CONFIG_PROXY_DEFAULT_LOAD_BALANCER: &str = "roundrobin";

// log config realted
const RUPID_CONFIG_PROXY_LOG_DEFAULT_TO_STD_OUT: bool = true;

// entrace config related
const RUPID_CONFIG_PROXY_ENTRANCE_DEFAULT_METHOD: &str = "ALL";

// middleware config related
// todo...

// load balancer config related
const RUPID_CONFIG_PROXY_LOAD_BALANCER_ROUND_ROBIN: &str = "roundrobin";
// todo...

// backend config related
const RUPID_CONFIG_DEFAULT_BACKEND_USE_SSL: bool = false;
const RUPID_CONFIG_DEFAULT_BACKEND_METHOD: &str = "PRESERVE";
