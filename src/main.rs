mod config;
pub mod constants;
mod error;
mod option;
mod types;

use anyhow::Result;
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
    info!("Hello, Rupid! with option: {:?}", &opt);

    // read yaml config
    let conf = Config::read_from_file(opt.config_filename.as_str())?;
    info!("with config: {:?}", conf);

    Ok(())
}
