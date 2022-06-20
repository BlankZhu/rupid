use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Destination {
    pub host: String,
    pub port: u16,
    pub weight: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StringMatch {
    pub exact: Option<String>,
    pub prefix: Option<String>,
    pub regex: Option<String>,
}
