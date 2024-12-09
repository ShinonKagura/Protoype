use std::collections::HashMap;
use anyhow::Result;
use crate::plugin_api::base::{Plugin, PluginType};
use crate::plugins::{
    PluginInstance,
    zip::ZipPlugin,
    sevenz::SevenZipPlugin,
    zstd::ZstdPlugin
};

pub struct PluginRegistry {
    plugins: HashMap<String, PluginInstance>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        PluginRegistry {
            plugins: HashMap::new(),
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        // Register built-in plugins
        self.register_plugin(PluginInstance::Zip(ZipPlugin::new()))?;
        self.register_plugin(PluginInstance::SevenZip(SevenZipPlugin::new()))?;
        self.register_plugin(PluginInstance::Zstd(ZstdPlugin::new()))?;
        Ok(())
    }

    pub fn register_plugin(&mut self, mut plugin: PluginInstance) -> Result<()> {
        plugin.initialize()?;
        let name = plugin.name().to_string();
        self.plugins.insert(name, plugin);
        Ok(())
    }

    pub fn get_plugins_by_type(&self, plugin_type: PluginType) -> Vec<PluginInstance> {
        self.plugins
            .values()
            .filter(|p| p.get_config().plugin_type == plugin_type)
            .cloned()
            .collect()
    }
}
