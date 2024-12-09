use std::any::Any;
use std::fmt;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use crate::plugin_api::types::{PluginType, PluginMetadata};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub name: String,
    pub description: String,
    pub version: String,
    pub plugin_type: PluginType,
}

#[derive(Debug)]
pub enum PluginError {
    NotImplemented,
    NotFound(String),
    AlreadyExists(String),
    InvalidInput(String),
    Other(String),
}

impl fmt::Display for PluginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginError::NotImplemented => write!(f, "Operation not implemented"),
            PluginError::NotFound(s) => write!(f, "Not found: {}", s),
            PluginError::AlreadyExists(s) => write!(f, "Already exists: {}", s),
            PluginError::InvalidInput(s) => write!(f, "Invalid input: {}", s),
            PluginError::Other(s) => write!(f, "Error: {}", s),
        }
    }
}

impl std::error::Error for PluginError {}

impl From<std::io::Error> for PluginError {
    fn from(error: std::io::Error) -> Self {
        PluginError::Other(error.to_string())
    }
}

pub trait Plugin: Send + Sync + Any {
    fn get_config(&self) -> &PluginConfig;
    fn metadata(&self) -> PluginMetadata;
    fn initialize(&mut self) -> Result<(), PluginError>;
    fn cleanup(&mut self) -> Result<(), PluginError>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait PluginFactory: Send + Sync {
    fn create(&self) -> Result<Box<dyn Plugin>, PluginError>;
}

// Required for dynamic loading
// Macro for plugin registration
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:expr) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut dyn $crate::plugin_api::base::Plugin {
            let plugin = $constructor();
            Box::into_raw(Box::new(plugin))
        }
    };
}
