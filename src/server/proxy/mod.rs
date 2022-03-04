mod backend;
mod entrance;
mod middleware;

use crate::config;

use self::backend::Backend;
use self::entrance::Entrance;

use axum::routing::*;
use axum::Router;
use hyper::{Body, Request, Response};
use std::convert::Infallible;
use tower::service_fn;

#[derive(Debug, Clone)]
pub struct Proxy {
    entrances: Vec<entrance::Entrance>,
    // middlewares: Vec<Box<dyn middleware::Middleware>>,
    backends: Vec<backend::Backend>,
}

impl Proxy {
    pub fn new(config: config::Proxy) -> Self {
        let mut entrances = Vec::new();
        // let mut middlewares = Vec::new();
        let mut backends = Vec::new();

        for e in config.entrances {
            let entrance = Entrance::new(e);
            entrances.push(entrance);
        }
        // todo: for m in config.middlewares...
        for b in config.backends {
            let backend = Backend::new(b);
            backends.push(backend);
        }

        Proxy {
            entrances,
            // middlewares,
            backends,
        }
    }

    pub fn generate_router(&self) -> Router<Body> {
        let mut router: Router<Body> = Router::new();

        for e in &self.entrances {
            for b in &self.backends {
                router = self.add_route(router, e.clone(), b.clone());
            }
        }

        router
    }

    fn add_route(&self, r: Router<Body>, e: Entrance, b: Backend) -> Router<Body> {
        match e.method.as_str() {
            "DELETE" => r.route(
                e.target.as_str(),
                delete_service(service_fn(|_: Request<Body>| async {
                    let res = Response::new(Body::from("Hi from `GET /`"));
                    Ok::<_, Infallible>(res)
                })),
            ),
            "GET" => r.route(
                e.target.as_str(),
                get_service(service_fn(|_: Request<Body>| async {
                    let res = Response::new(Body::from("Hi from `GET /`"));
                    Ok::<_, Infallible>(res)
                })),
            ),
            "HEAD" => r.route(
                e.target.as_str(),
                head_service(service_fn(|_: Request<Body>| async {
                    let res = Response::new(Body::from("Hi from `GET /`"));
                    Ok::<_, Infallible>(res)
                })),
            ),
            "OPTIONS" => r.route(
                e.target.as_str(),
                options_service(service_fn(|_: Request<Body>| async {
                    let res = Response::new(Body::from("Hi from `GET /`"));
                    Ok::<_, Infallible>(res)
                })),
            ),
            "PATCH" => r.route(
                e.target.as_str(),
                patch_service(service_fn(|_: Request<Body>| async {
                    let res = Response::new(Body::from("Hi from `GET /`"));
                    Ok::<_, Infallible>(res)
                })),
            ),
            "POST" => r.route(
                e.target.as_str(),
                post_service(service_fn(|_: Request<Body>| async {
                    let res = Response::new(Body::from("Hi from `GET /`"));
                    Ok::<_, Infallible>(res)
                })),
            ),
            "PUT" => r.route(
                e.target.as_str(),
                put_service(service_fn(|_: Request<Body>| async {
                    let res = Response::new(Body::from("Hi from `GET /`"));
                    Ok::<_, Infallible>(res)
                })),
            ),
            "TRACE" => r.route(
                e.target.as_str(),
                trace_service(service_fn(|_: Request<Body>| async {
                    let res = Response::new(Body::from("Hi from `GET /`"));
                    Ok::<_, Infallible>(res)
                })),
            ),
            _ => r.route(
                e.target.as_str(),
                any_service(service_fn(|_: Request<Body>| async {
                    let res = Response::new(Body::from("Hi from `GET /`"));
                    Ok::<_, Infallible>(res)
                })),
            ),
        }
    }
}
