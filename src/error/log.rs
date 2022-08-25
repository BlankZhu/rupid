use std::fmt;

#[derive(Debug, Clone)]
pub struct LogInitializeError {
    detail: String,
}

impl fmt::Display for LogInitializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_msg = format!("failed to initialize Rupid's logging function, caused by: {}", &self.detail);
        write!(f, "{}", err_msg)
    }
}

impl LogInitializeError {
    pub fn new(cause: &str) -> Self {
        return LogInitializeError { detail: cause.to_string() }
    }
}