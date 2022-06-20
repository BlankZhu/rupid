use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::Destination;

/// describes the possible middlewares, including `tcp`, `tls` and `http`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Middlewares {
    /// TCP middlewares, default empty; todo
    pub tcp: Option<TCP>,
    /// TLS middlewares, default empty; todo
    pub tls: Option<TLS>,
    /// HTTP middlewares, default empty.
    pub http: Option<HTTP>,
}

/// describes the possible middlewares for TCP connections, TODO.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TCP {}

/// describes the possible middlewares for TLS connections, TODO.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TLS {}

/// describes the possible middlewares for HTTP connections.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HTTP {
    /// HTTP redirect middlewares, default empty.
    pub redirect: Option<Vec<Redirect>>,

    /// HTTP rewrite middlewares, default empty.
    pub rewrite: Option<Vec<Rewrite>>,

    /// HTTP timeout middlewares, default empty.
    pub timeout: Option<Vec<Timeout>>,

    /// HTTP retries middlewares, default empty.
    pub retries: Option<Vec<Retries>>,

    /// HTTP fault middlewares, default empty.
    pub fault: Option<Vec<Fault>>,

    /// HTTP mirror middlewares, default empty.
    pub mirror: Option<Vec<Mirror>>,

    /// HTTP header middlewares, default empty.
    pub header: Option<Vec<Header>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Redirect {
    /// Name of the redirect middleware.
    pub name: String,

    /// Redirect uri.
    pub uri: String,

    /// Redirect authority.
    pub authority: String,

    /// Redirect port.
    pub port: u16,

    /// Redirect scheme.
    pub scheme: String,

    /// Redirect returned http response's status code.
    pub redirect_code: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rewrite {
    /// Name of the rewrite middleware.
    pub name: String,

    /// Rewrite uri.
    pub uri: String,

    /// Rewrite authority.
    pub authority: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Timeout {
    /// Name of the timeout middleware.
    pub name: String,

    /// Timeout for writing to backend services, in `ms`.
    /// If the value is `0` or unset, there won't be any timeout.
    pub write_request_timeout: u128,

    /// Timeout for reading from backend services, in `ms`.
    /// If the value is `0` or unset, there won't be any timeout.
    pub read_response_timeout: u128,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Retries {
    /// Name of the retries middleware.
    pub name: String,

    /// Number of retries to be allowed for a given request.
    pub attempts: u16,

    /// Timeout per attempt for a given request.
    pub per_try_timeout: u128,

    /// Retry conditions, include: `4XX`, `5XX`, `TCP_ERROR`.
    /// All the conditions are combined in an `OR` logic.
    pub retry_on: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fault {
    /// Name of the fault middleware.
    pub name: String,

    /// Delay fault option, independent to `abort`.
    pub delay: Option<Delay>,

    /// Abort fault option, independent to `delay`.
    pub abort: Option<Abort>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Delay {
    /// from 0.0 to 1.0, those not in range will be cut to 0.0 or 1.0.
    pub percentage: f64,

    /// delay time, in ms.
    pub time: u128,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Abort {
    /// from 0.0 to 1.0, those not in range will be cut to 0.0 or 1.0.
    pub percentage: f64,

    /// Abort returned HTTP status code.
    pub http_status: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mirror {
    /// Name of the mirror middleware.
    pub name: String,

    /// mirror traffic HTTP destination.
    pub destination: Destination,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Header {
    /// Name of the header middleware.
    pub name: String,

    /// Operations on client requests, applied by list order.
    pub reqeust: Option<Vec<Action>>,

    /// Operations on server responses, applied by list order.
    pub response: Option<Vec<Action>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Action {
    /// set header with given field & value
    pub set: Option<HashMap<String, String>>,

    /// add header with value on given field
    pub add: Option<HashMap<String, String>>,

    /// remove header by given field
    pub remove: Option<Vec<String>>,
}
