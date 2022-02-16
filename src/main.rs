mod config;
mod constants;
mod error;
mod option;
mod server;
mod types;

use clap::Clap;
use config::Config;
use futures::TryStreamExt;
use http;
use log::{error, info};
use option::Option;
use warp::Filter;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "trace");
    }
    pretty_env_logger::init();

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

    let routes = bing_search_api();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn bing_search_api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path::param::<String>())
        .and(warp::header::headers_cloned())
        .and(warp::body::stream())
        .and_then(bing_search_callback)
}

async fn bing_search_callback(
    search: String,
    headers: http::HeaderMap,
    body: impl futures::Stream<Item = Result<impl bytes::buf::Buf, warp::Error>> + Send + 'static,
) -> Result<http::Response<warp::hyper::body::Body>, warp::Rejection> {
    let request = bing_build_search_request(search, headers, body);
    // todo: add tokio::time::timeout
    let client = warp::hyper::Client::new();

    if let Ok(proxy_response) = client.request(request).await {
        let proxy_status = proxy_response.status();
        let proxy_headers = proxy_response.headers().clone();
        let proxy_body = proxy_response.into_body();

        let mut response = http::Response::new(proxy_body);
        *response.status_mut() = proxy_status;
        *response.headers_mut() = proxy_headers;

        Ok(response)
    } else {
        Ok(http::Response::builder()
            .status(http::StatusCode::SERVICE_UNAVAILABLE)
            .body("proxy server unavailable".into())
            .unwrap())
    }
}

fn bing_build_search_request(
    search: String,
    headers: http::HeaderMap,
    body: impl futures::Stream<Item = Result<impl bytes::Buf, warp::Error>> + Send + 'static,
) -> http::Request<warp::hyper::Body> {
    let uri = format!("http://cn.bing.com:80/search?q={}", search.as_str());
    let body = body.map_ok(|mut buf| buf.copy_to_bytes(buf.remaining()));
    let mut request = http::Request::new(warp::hyper::Body::wrap_stream(body));
    *request.method_mut() = http::method::Method::GET;
    *request.uri_mut() = uri.parse().unwrap();
    *request.headers_mut() = headers;
    request
        .headers_mut()
        .insert(http::header::HOST, "cn.bing.com".parse().unwrap());
    request
}
