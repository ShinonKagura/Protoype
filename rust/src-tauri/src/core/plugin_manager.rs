use crate::plugin_api::base::{Plugin, PluginError};
use crate::plugin_api::types::PluginMetadata;
use crate::plugins::registry::PluginRegistry;

pub struct PluginManager {
    registry: PluginRegistry,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            registry: PluginRegistry::new(),
        }
    }

    pub fn register_default_plugins(&mut self) -> Result<(), PluginError> {
        let zip_plugin = Box::new(crate::plugins::zip::ZipPlugin::new());
        let zip_name = zip_plugin.get_config().name.clone();
        self.registry.register(zip_name, zip_plugin)?;

        let sevenz_plugin = Box::new(crate::plugins::sevenz::SevenZipPlugin::new());
        let sevenz_name = sevenz_plugin.get_config().name.clone();
        self.registry.register(sevenz_name, sevenz_plugin)?;

        Ok(())
    }

    pub fn get_plugin(&self, name: &str) -> Option<&Box<dyn Plugin>> {
        self.registry.get_plugin(name)
    }

    pub fn get_plugin_mut(&mut self, name: &str) -> Option<&mut Box<dyn Plugin>> {
        self.registry.get_plugin_mut(name)
    }

    pub fn list_plugins(&self) -> Vec<PluginMetadata> {
        self.registry.list_plugins()
    }

    pub fn cleanup(&mut self) -> Result<(), PluginError> {
        self.registry.cleanup_all()
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        if let Err(e) = self.cleanup() {
            eprintln!("Error during plugin cleanup: {}", e);
        }
    }
}
