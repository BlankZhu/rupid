mod config;
mod option;
mod server;
mod types;

use clap::Clap;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // parse command line options
    // ...

    // read config
    // ...

    // setup proxy engine
    // ...

    Ok(())
}
