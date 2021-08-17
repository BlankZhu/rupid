use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Handler {
    pub entry_point: String,
    pub manual_routing: bool,
    pub terminating_status_code: u16,
    pub terminating: bool,
}

impl Handler {
    pub fn set_defaults(&mut self) {
        self.entry_point = "rupid".to_string();
        self.terminating_status_code = http::status::StatusCode::SERVICE_UNAVAILABLE.as_u16();
    }

    // handle http
}