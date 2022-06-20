mod config;
pub mod constants;
mod error;
mod option;
mod types;

use log::info;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    info!("Hello, Rupid!");
}
