use crate::config;
use crate::constants::*;

use futures::Future;
use http::uri;
use hyper::client::HttpConnector;
use hyper::service::Service;
use hyper::{http, Body, Client, Request, Response, StatusCode};
use log::{debug, error, warn};
use std::convert::Infallible;
use std::pin::Pin;
use std::str::FromStr;
use std::task::{Context, Poll};
use std::time::Duration;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type BackendResult<T> = std::result::Result<T, GenericError>;

#[derive(Debug, Clone)]
pub struct Backend {
    name: String,
    host: String,
    port: u16,
    use_ssl: bool,
    certificate: String,
    method: http::Method,
    target: String,

    client: Client<HttpConnector>,
}

impl Backend {
    pub fn new(config: config::Backend) -> Self {
        let port: u16 = config.port.map_or(
            RUPID_CONFIG_PROXY_BACKEND_DEFAULT_PORT,
            |port_str: String| match port_str.parse::<u16>() {
                Ok(p) => p,
                Err(e) => {
                    warn!("invalid port: {}, using 80", &port_str);
                    RUPID_CONFIG_PROXY_BACKEND_DEFAULT_PORT
                }
            },
        );

        let timeout: Duration = config.timeout.map_or(
            RUPID_CONFIG_PROXY_BACKEND_DEFAULT_TIMEOUT,
            |timeout: u64| Duration::from_millis(timeout),
        );

        let use_ssl: bool = config.use_ssl.map_or(false, |b| b);

        let certificate: String = config.certificate.map_or("".to_string(), |c| c);

        // fixme: handle PRESERVE as default?
        let method: http::Method = config
            .method
            .map_or(
                http::Method::GET,
                |method_str: String| match http::Method::from_str(method_str.as_str()) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("invalid method: {}", e);
                        http::Method::GET
                    }
                },
            );

        Backend {
            name: config.name,
            host: config.host,
            port,
            use_ssl,
            certificate,
            method,
            target: config.target,

            client: Client::builder().build_http(), // fixme: integrate it with timeout, keep-alive & concurrency limit
        }
    }

    // todo: add an extra service layer
    pub async fn request(&self, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        // todo: convert Error into HTTP 5XX response
        let res = self.exec(req).await;
        match res {
            Ok(res) => return Ok(res),
            Err(err) => {
                let err_resp = Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(format!("{}", err)))
                    .unwrap();
                error!("failed to execute HTTP request, detail: {}", err);
                return Ok(err_resp);
            }
        }
    }

    async fn exec(&self, req: Request<Body>) -> BackendResult<Response<Body>> {
        let scheme = match self.use_ssl {
            true => "https",
            false => "http",
        };
        let query = req.uri().query().map_or("", |q| q).to_string();
        let mut headers = req.headers().clone();
        // fixme: carefully select header fields to override
        headers.insert(http::header::HOST, self.host.as_str().parse().unwrap());
        let body = hyper::body::to_bytes(req).await?;
        let body = Body::from(body);

        let new_uri = uri::Builder::new()
            .scheme(scheme)
            .authority(format!("{}:{}", self.host.clone(), self.port.clone()))
            .path_and_query(format!("{}?{}", self.target.clone(), query))
            .build()?;

        debug!("using uri: {}", new_uri);
        debug!("using headers: {:?}", headers);
        let mut new_request = Request::builder()
            .method(self.method.clone())
            .uri(new_uri)
            .body(body)?;
        *new_request.headers_mut() = headers;

        let response = self.client.request(new_request).await?;
        Ok(response)
    }
}

impl Service<Request<Body>> for Backend {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let this = self.clone();
        Box::pin(async move { this.request(req).await })
    }
}
