use serde::{Deserialize, Serialize};

use super::*;

const PROVIDER_NAME: &str = "file";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileProvider {
    pub directory: String,  // load dynamic configuration from one or more config files in a directory
    pub watch: bool,        // if watch the provider or not  
    pub filename: String,   // load dynamic configuration from a file
}

impl ProviderTrait for FileProvider {
    fn init(&mut self) -> Result<(), ProviderInitError> {
        Ok(())
    }

    fn provide(&self, sender: mpsc::Sender<dynamic::Message>) -> Result<(), ProvideError> {
        todo!()
    }
}

impl FileProvider {
    pub fn set_defaults(&mut self) {
        self.watch = true;
        self.filename = "".to_string();
    }

    pub fn build_configuration(&mut self) {
        
    }
}
