use std::env;

use crate::config::log as rupid_log_config;
use crate::error::log as rupid_log_error;

pub fn initialize_logger(config: &rupid_log_config::Log) -> Result<(), rupid_log_error::LogInitializeError> {
    // set log level
    let log_level = match config.level.as_deref() {
        // Some(level) => level.clone(),
        // None => "info".to_string(),
        Some("debug") => "debug",
        Some("info") => "info",
        Some("warn") => "warn",
        Some("error") => "error",
        Some("fatal") => "fatal",
        None => "info",
        Some(_) => "info",
    };
    env::set_var("RUST_LOG", log_level);

    // set stderr to stdout
    todo!();

    // setup file out
    todo!();
    
    Ok(())
}