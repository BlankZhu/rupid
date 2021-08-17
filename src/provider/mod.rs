pub mod file_provider;

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use crate::config::dynamic;

#[derive(Debug, Clone)]
pub struct ProvideError {
    pub detail: String
}

#[derive(Debug, Clone)]
pub struct ProviderInitError {
    pub detail: String
}

pub trait ProviderTrait {
    fn provide(&self, sender: mpsc::Sender<dynamic::Message>) -> Result<(), ProvideError>;
    fn init(&mut self) -> Result<(), ProviderInitError>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Provider {
    FileProvider(file_provider::FileProvider),
}
