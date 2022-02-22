mod config;
mod constants;
mod error;
mod option;
mod server;
mod types;

use clap::Parser;
use config::Config;
use futures::TryStreamExt;
use http::{self, HeaderMap};
use log::{error, info};
use option::Option;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let opt = Option::parse();
    let conf = match Config::load_from_yaml_file(&opt.config_filename) {
        Ok(c) => c,
        Err(e) => {
            error!(
                "failed to load rupid config from {}, detail: {}",
                opt.config_filename, e
            );
            return;
        }
    };
    info!("using config: {:?}", conf);

    let app = axum::Router::new().route("/:word", axum::routing::get(httpbin_org_handler));
    axum::Server::bind(&"0.0.0.0:7777".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn httpbin_org_handler(
    method: axum::http::Method,
    axum::extract::Path(word): axum::extract::Path<String>,
    headers: HeaderMap,
    axum::extract::RawBody(body): axum::extract::RawBody,
) -> http::Response<hyper::body::Body> {
    let request = build_httpbin_org_request(method, format!("{}", word.as_str()), headers, body);
    let client = hyper::Client::new();

    if let Ok(upstream_response) = client.request(request).await {
        let upstream_status = upstream_response.status();
        let upstream_headers = upstream_response.headers().clone();
        let upstream_body = upstream_response.into_body();

        let mut response = http::Response::new(upstream_body);
        *response.status_mut() = upstream_status;
        *response.headers_mut() = upstream_headers;
        response
    } else {
        http::Response::builder()
            .status(http::StatusCode::SERVICE_UNAVAILABLE)
            .body("proxy server unavailable".into())
            .unwrap()
    }
}

fn build_httpbin_org_request(
    method: http::Method,
    path: String,
    headers: HeaderMap,
    body: hyper::body::Body,
) -> http::Request<hyper::body::Body> {
    let uri = format!("http://httpbin.org:80/{}", path.as_str());
    let mut request = http::Request::new(body);
    *request.method_mut() = method;
    *request.uri_mut() = uri.parse().unwrap();
    *request.headers_mut() = headers;
    request
        .headers_mut()
        .insert(http::header::HOST, "httpbin.org".parse().unwrap());

    request
}
