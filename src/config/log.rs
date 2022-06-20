use serde::{Deserialize, Serialize};

/// Describes the config about logging
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Log {
    /// Log level, values include: trace, debug, info, warn, error and fatal.
    /// The default log level is `info`.
    pub level: Option<String>,

    /// Standard output, tells the rupid if it should log into `stdout`.
    pub std: Option<Std>,

    /// File out, tells the rupid if it should log into given path.
    pub file: Option<File>,
}

/// Config about logging to standard output.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Std {
    /// default is `true`
    pub enabled: Option<bool>,
}

/// Config about logging to file output.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
    /// default is `false`
    pub enabled: Option<bool>,

    /// path of the log file, default is `~` ($PWD)
    pub path: Option<String>,
}
