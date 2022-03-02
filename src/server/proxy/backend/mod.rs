use crate::config;
use crate::constants;
use futures::TryStreamExt;
use hyper::{client, http, Client};
use log::{error, warn};
use std::str::FromStr;
use std::time;

#[derive(Debug)]
pub struct Backend {
    name: String,
    host: String,
    port: u16,
    use_ssl: bool,
    certificate: String,
    method: http::Method,
    target: String,

    client: Client<client::HttpConnector>,
}

impl Backend {
    pub fn new(config: config::Backend) -> Self {
        let port: u16 = config.port.map_or(
            constants::RUPID_CONFIG_BACKEND_DEFAULT_PORT,
            |port_str: String| match port_str.parse::<u16>() {
                Ok(p) => p,
                Err(e) => {
                    warn!("invalid port: {}, using 80", &port_str);
                    constants::RUPID_CONFIG_BACKEND_DEFAULT_PORT
                }
            },
        );

        let timeout: time::Duration = config.timeout.map_or(
            constants::RUPID_CONFIG_BACKEND_DEFAULT_TIMEOUT,
            |timeout: u64| time::Duration::from_millis(timeout),
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

    async fn callback(
        &self,
        method: http::Method,
        path: warp::path::FullPath,
        headers: http::HeaderMap,
        body: impl futures::Stream<Item = Result<impl bytes::Buf, warp::Error>> + Send + 'static,
    ) -> http::Response<hyper::Body> {
        let uri = format!(
            "http://{}:{}{}",
            self.host.clone().as_str(),
            self.port.clone(),
            path.as_str()
        );
        let body = body.map_ok(|mut buf| buf.copy_to_bytes(buf.remaining()));
        let mut request = http::Request::new(warp::hyper::Body::wrap_stream(body));
        *request.method_mut() = method;
        *request.uri_mut() = uri.parse().unwrap();
        *request.headers_mut() = headers;
        request
            .headers_mut()
            .insert(http::header::HOST, self.host.as_str().parse().unwrap());

        let response = self.client.request(request).await;
        let ret = match response {
            Ok(response) => response,
            Err(e) => http::Response::builder()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("{}", e).into())
                .unwrap(),
        };

        return ret;
    }
}
