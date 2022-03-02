mod backend;
mod entrance;
mod middleware;

pub struct Proxy {
    entrances: Vec<entrance::Entrance>,
    middlewares: Vec<Box<dyn middleware::Middleware>>,
    backends: Vec<backend::Backend>,
}

impl Proxy {
    pub fn new() -> Self {
        todo!()
    }

    // todo: chain entrances -> middlewares -> backends
    fn chain() {
        todo!()
    }

    // todo: return a warp::filter
    pub fn generate_filter() {
        todo!()
    }
}
