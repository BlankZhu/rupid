mod config;
mod constants;
mod error;
mod option;
mod server;
mod types;

use clap::Clap;
use config::Config;
use log::error;
use option::Option;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "todos=info");
    }
    pretty_env_logger::init();

    let opt = Option::parse();
    let conf = match Config::load_from_yaml_file(&opt.config_filename) {
        Ok(c) => c,
        Err(e) => {
            error!(
                "failed to load rupid config from {}, detail: {}",
                opt.config_filename, e
            );
            return;
        }
    };

    // todo: setup rupid server...
}
