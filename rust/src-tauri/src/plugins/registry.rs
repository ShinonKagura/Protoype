use std::collections::HashMap;
use anyhow::Result;
use crate::plugin_api::base::{Plugin, PluginError};
use crate::plugin_api::types::PluginMetadata;

pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        if self.plugins.contains_key(&name) {
            return Err(PluginError::AlreadyExists(name));
        }
        self.plugins.insert(name, plugin);
        Ok(())
    }

    pub fn get_plugin(&self, name: &str) -> Option<&Box<dyn Plugin>> {
        self.plugins.get(name)
    }

    pub fn get_plugin_mut(&mut self, name: &str) -> Option<&mut Box<dyn Plugin>> {
        self.plugins.get_mut(name)
    }

    pub fn list_plugins(&self) -> Vec<PluginMetadata> {
        self.plugins.values()
            .map(|plugin| plugin.metadata())
            .collect()
    }

    pub fn cleanup_all(&mut self) -> Result<(), PluginError> {
        for plugin in self.plugins.values_mut() {
            plugin.cleanup()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugins::zip::ZipPlugin;

    #[test]
    fn test_plugin_registry() {
        let mut registry = PluginRegistry::new();
        let plugin = Box::new(ZipPlugin::new());
        registry.register("ZIP Plugin".to_string(), plugin).unwrap();
        assert!(registry.get_plugin("ZIP Plugin").is_some());
    }
}
