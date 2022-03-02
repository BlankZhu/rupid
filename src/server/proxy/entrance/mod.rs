use std::pin::Pin;

use crate::config;
use hyper::http;
use warp::Filter;

#[derive(Debug)]
pub struct Entrance {
    name: String,
    host: String,
    method: http::Method,
    target: String,
}

impl Entrance {
    pub fn new(config: config::Entrance) -> Self {
        todo!()
    }
}
