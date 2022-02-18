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

    let routes = httpbin_org_api();
    let service = warp::service(routes);
    let make_service = warp::hyper::service::make_service_fn(move |_| async move {
        let service = tower::ServiceBuilder::new()
            .timeout(std::time::Duration::from_millis(10))
            .service(service);
        Ok::<_, std::convert::Infallible>(service)
    });
    let server = warp::hyper::Server::bind(&([127, 0, 0, 1], 7777).into()).serve(make_service);
    server.await;
}

fn httpbin_org_api(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone + Copy {
    warp::method()
        .and(warp::filters::path::full())
        .and(warp::header::headers_cloned())
        .and(warp::body::stream())
        .and_then(httpbin_org_callback)
}

async fn httpbin_org_callback(
    method: http::Method,
    path: warp::filters::path::FullPath,
    headers: http::HeaderMap,
    body: impl futures::Stream<Item = Result<impl bytes::buf::Buf, warp::Error>> + Send + 'static,
) -> Result<http::Response<warp::hyper::body::Body>, warp::Rejection> {
    let request = httpbin_org_build_request(method, format!("{}", path.as_str()), headers, body);
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

fn httpbin_org_build_request(
    method: http::Method,
    path: String,
    headers: http::HeaderMap,
    body: impl futures::Stream<Item = Result<impl bytes::Buf, warp::Error>> + Send + 'static,
) -> http::Request<warp::hyper::Body> {
    let uri = format!("http://httpbin.org:80/{}", path.as_str());
    let body = body.map_ok(|mut buf| buf.copy_to_bytes(buf.remaining()));
    let mut request = http::Request::new(warp::hyper::Body::wrap_stream(body));
    *request.method_mut() = method;
    *request.uri_mut() = uri.parse().unwrap();
    *request.headers_mut() = headers;
    request
        .headers_mut()
        .insert(http::header::HOST, "httpbin.org".parse().unwrap());
    request
}
