mod api;
mod config;
mod constant;
mod log;
mod option;
mod pipeline;
mod plugin;
mod proxy;
mod router;
mod server;
mod service;
mod types;

use std::io;
use clap::Clap;

#[tokio::main]
async fn main() -> io::Result<()> {
    // parse command line options
    let opt = option::Options::parse();
    
    // read config
    // ...

    // setup proxy engine
    // ...

    Ok(())
}
