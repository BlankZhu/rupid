use crate::error::LoadYamlConfigError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub servers: Vec<Server>,
}

impl Config {
    pub fn load_from_yaml_file(filename: &String) -> Result<Config, LoadYamlConfigError> {
        std::fs::File::open(filename)
            .map_err(|err| LoadYamlConfigError {
                filename: filename.clone(),
                detail: format!("{}", err),
            })
            .and_then(|file| {
                serde_yaml::from_reader(file).map_err(|err| LoadYamlConfigError {
                    filename: filename.clone(),
                    detail: format!("{}", err),
                })
            })
    }

    // more to add, like load_from_toml_file...
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub address: Option<String>,
    pub port: Option<u16>,
    pub timeout: Option<u64>, // in micro second
    pub use_ssl: Option<bool>,
    pub private_key: Option<String>,
    pub certificate: Option<String>, // todo: certificate bundle
    pub proxies: Vec<Proxy>,
    pub log: Option<Log>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Proxy {
    pub entrances: Vec<Entrance>,
    pub middlewares: Option<Vec<Middleware>>,
    pub backends: Option<Vec<Backend>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Log {
    pub to_std_out: Option<bool>,
    pub to_file: Option<bool>,        // not avaliable now
    pub log_filename: Option<String>, // not avaliable now
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entrance {
    pub name: String,
    pub host: Option<String>,
    pub method: Option<String>, // GET, POST, ... and `ALL`
    pub target: String,
    // todo: more filter options to add...
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Middleware {
    pub name: String,
    pub parameters: Option<String>,
    // todo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Backend {
    pub name: String,
    pub host: String,
    pub port: Option<String>,
    pub timeout: Option<u64>, // in micro second
    pub use_ssl: Option<bool>,
    pub certificate: Option<String>,
    pub method: Option<String>, // GET, POST, ... and `PRESERVE`
    pub target: String,
    pub keep_alive: bool,
    pub pool_size: u64,
}
