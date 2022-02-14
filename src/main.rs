mod config;
mod constants;
mod error;
mod option;
mod server;
mod types;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // parse command line options
    // ...

    // read config
    // ...

    // setup proxy engine
    // ...

    Ok(())
}
