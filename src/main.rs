mod api;
mod config;
mod engine;
mod healthcheck;
mod ip;
mod log;
mod middleware;
mod option;
mod ping;
mod rule;
mod server;
mod tcp;
mod tls;
mod types;
mod udp;

use std::io;
use clap::Clap;

#[tokio::main]
async fn main() -> io::Result<()> {
    // parse command line options
    let opt = option::Options::parse();
    
    // read config
    let conf = config::RupidConfig::load_from_yaml_file(&opt.config)?;

    // setup proxy engine
    let _ = engine::RupidEngine::new(&conf);

    Ok(())
}
