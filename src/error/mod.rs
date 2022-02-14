use std::{error, fmt};

#[derive(Debug)]
pub struct LoadYamlConfigError {
    pub filename: String,
    pub detail: String,
}

impl fmt::Display for LoadYamlConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LoadYamlConfigError: cannot load `{}`, detail: {}",
            self.filename, self.detail
        )
    }
}

impl error::Error for LoadYamlConfigError {}
