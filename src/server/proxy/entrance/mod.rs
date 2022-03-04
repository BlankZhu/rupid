use crate::config;
use crate::constants::*;
use futures::Future;
use hyper::http;
use hyper::{Body, Request, Response};
use log::{debug, warn};
use std::pin::Pin;
use std::str::FromStr;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Debug, Clone)]
pub struct Entrance {
    pub name: String,
    pub host: String,
    pub method: String,
    pub target: String,
}

impl Entrance {
    pub fn new(config: config::Entrance) -> Self {
        let host = config.host.map_or("".to_string(), |s| s.to_string());
        let method = config.method.map_or(
            RUPID_CONFIG_PROXY_ENTRANCE_DEFAULT_METHOD.to_string(),
            |method_str: String| match http::Method::from_str(method_str.as_str()) {
                Ok(_) => method_str,
                Err(e) => {
                    warn!("invalid method: {}", e);
                    RUPID_CONFIG_PROXY_BACKEND_DEFAULT_METHOD.to_string()
                }
            },
        );

        Entrance {
            name: config.name,
            host,
            method,
            target: config.target,
        }
    }
}

impl<S> Layer<S> for Entrance {
    type Service = EntranceMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        EntranceMiddleware {
            name: self.name.clone(),
            host: self.host.clone(),
            method: self.method.clone(),
            target: self.target.clone(),
            inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntranceMiddleware<S> {
    pub name: String,
    pub host: String,
    pub method: String,
    pub target: String,

    inner: S,
}

impl<S> Service<Request<Body>> for EntranceMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        debug!("request header: {:?}", req.headers());
        debug!("req_uri: {}", req.uri());
        // rewrite the request path
        let new_target = cut(req.uri().path().to_string(), self.target.clone());
        let query = req.uri().query().map_or("", |q| q);
        debug!("new_target: {}", new_target);
        let new_uri = http::Uri::builder()
            .path_and_query(format!("{}?{}", new_target, query))
            .build()
            .unwrap();
        *req.uri_mut() = new_uri;
        debug!("new_uri: {}", req.uri());

        let future = self.inner.call(req);
        Box::pin(async move {
            let response = future.await?;
            Ok(response)
        })
    }
}

fn longest_common_prefix(strs: &[String]) -> &str {
    if strs.is_empty() {
        return "";
    }

    let first = &strs[0];

    for (i, byte) in first.as_bytes().iter().enumerate() {
        for string in strs {
            if string.as_bytes().get(i) != Some(byte) {
                let mut end = i;
                while !first.is_char_boundary(end) {
                    end -= 1;
                }

                return &first[0..end];
            }
        }
    }

    first
}

fn cut(to_cut: String, pattern: String) -> String {
    // todo: handle `*` (all incoming segment) and `:` (one segment)
    // currently this only works on static path

    let zip = vec![to_cut.clone(), pattern];
    let lcp = longest_common_prefix(&zip);
    let mut ret = to_cut.trim_start_matches(lcp).to_string();
    if !ret.starts_with("/") {
        ret = "/".to_string() + &ret;
    }
    ret
}
