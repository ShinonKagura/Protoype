use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use crate::plugin_api::base::{Plugin, PluginError};
use crate::plugin_api::compression::CompressionPlugin;
use crate::plugin_api::types::CompressionOptions;
use anyhow::Result;

mod loader;
pub use loader::PluginLoader;

pub mod registry;
pub mod sevenz;
pub mod zip;
pub mod template;

pub struct PluginInstance {
    plugin: Box<dyn Plugin>,
    initialized: bool,
}

impl PluginInstance {
    pub fn new(plugin: Box<dyn Plugin>) -> Self {
        Self {
            plugin,
            initialized: false,
        }
    }

    pub fn initialize(&mut self) -> Result<(), PluginError> {
        if !self.initialized {
            self.plugin.initialize()?;
            self.initialized = true;
        }
        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<(), PluginError> {
        if self.initialized {
            self.plugin.cleanup()?;
            self.initialized = false;
        }
        Ok(())
    }

    pub fn as_compression_plugin(&self) -> Option<&dyn CompressionPlugin> {
        self.plugin.as_any().downcast_ref::<Box<dyn CompressionPlugin>>()
            .map(|p| p.as_ref())
    }

    pub fn compress(
        &self,
        input_files: &[PathBuf],
        output_file: &PathBuf,
        options: &CompressionOptions,
    ) -> Result<(), PluginError> {
        if let Some(plugin) = self.as_compression_plugin() {
            plugin.compress(input_files, output_file, options)
        } else {
            Err(PluginError::Other("Plugin does not support compression".to_string()))
        }
    }

    pub fn decompress(
        &self,
        archive_file: &PathBuf,
        output_dir: &PathBuf,
        overwrite: bool,
    ) -> Result<(), PluginError> {
        if let Some(plugin) = self.as_compression_plugin() {
            plugin.decompress(archive_file, output_dir, overwrite)
        } else {
            Err(PluginError::Other("Plugin does not support decompression".to_string()))
        }
    }
}

impl Drop for PluginInstance {
    fn drop(&mut self) {
        if self.initialized {
            if let Err(e) = self.cleanup() {
                eprintln!("Error during plugin cleanup: {}", e);
            }
        }
    }
}

pub struct PluginRegistry {
    plugins: HashMap<String, Arc<Mutex<PluginInstance>>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        let mut instance = PluginInstance::new(plugin);
        instance.initialize()?;
        self.plugins.insert(name, Arc::new(Mutex::new(instance)));
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<Arc<Mutex<PluginInstance>>> {
        self.plugins.get(name).cloned()
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Arc<Mutex<PluginInstance>>> {
        self.plugins.get_mut(name)
    }

    pub fn initialize(&mut self) -> Result<(), PluginError> {
        for plugin in self.plugins.values() {
            if let Ok(mut guard) = plugin.lock() {
                guard.initialize()?;
            }
        }
        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<(), PluginError> {
        for plugin in self.plugins.values() {
            if let Ok(mut guard) = plugin.lock() {
                guard.cleanup()?;
            }
        }
        Ok(())
    }

    pub fn list_plugins(&self) -> Vec<&str> {
        self.plugins.keys().map(|k| k.as_str()).collect()
    }
}
