mod config;
pub mod constants;
mod error;
mod option;
mod types;

use anyhow::{Context, Result};
use clap::Parser;

use config::Config;
use log::info;
use option::Option;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize log
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    // parse command line options
    let opt = Option::parse();

    // read yaml config
    let conf = Config::read_from_file(opt.config_filename.as_str()).with_context(|| {
        format!(
            "Failed to read Rupid config from {}",
            opt.config_filename.as_str()
        )
    })?;

    info!("Rupid using config: {}", conf);

    Ok(())
}
