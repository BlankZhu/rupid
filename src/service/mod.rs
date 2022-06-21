use std::collections::HashSet;

pub mod http;
pub mod local;
pub mod tcp;
pub mod tls;

pub struct Service {
    name: String,
    hosts: HashSet<String>,
    // todo
}
