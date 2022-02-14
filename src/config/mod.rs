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
    pub port: u16,
    pub use_ssl: bool,
    pub private_key: String,
    pub certificate: String,
    pub proxies: Vec<Proxy>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Proxy {
    pub entrances: Vec<Entrance>,
    pub middlewares: Vec<Middleware>,
    pub load_balancer: LoadBalancer,
    pub backends: Vec<Backend>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entrance {
    pub name: String,
    pub host: String,
    pub method: String,
    pub target: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Middleware {
    pub name: String,
    // todo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoadBalancer {
    pub name: String,
    // todo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Backend {
    pub name: String,
    pub host: String,
    pub port: String,
    pub use_ssl: bool,
    pub method: String,
    pub target: String,
}
