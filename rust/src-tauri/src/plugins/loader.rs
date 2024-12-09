use std::path::PathBuf;
use anyhow::Result;
use libloading::{Library, Symbol};
use crate::plugin_api::base::Plugin;
use crate::plugins::PluginInstance;

pub struct PluginLoader;

impl PluginLoader {
    pub fn load_plugin(path: PathBuf) -> Result<PluginInstance> {
        unsafe {
            let lib = Library::new(path)?;
            let constructor: Symbol<fn() -> Box<dyn Plugin>> = lib.get(b"_plugin_create")?;
            let plugin = constructor();
            Ok(PluginInstance::new(plugin))
        }
    }

    pub fn discover_plugins(plugin_dir: PathBuf) -> Result<Vec<PluginInstance>> {
        let mut plugins = Vec::new();
        
        if !plugin_dir.exists() {
            return Ok(plugins);
        }

        for entry in std::fs::read_dir(plugin_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && Self::is_plugin_library(&path) {
                if let Ok(plugin) = Self::load_plugin(path) {
                    plugins.push(plugin);
                }
            }
        }

        Ok(plugins)
    }

    fn is_plugin_library(path: &PathBuf) -> bool {
        if let Some(ext) = path.extension() {
            #[cfg(target_os = "windows")]
            return ext == "dll";
            #[cfg(target_os = "linux")]
            return ext == "so";
            #[cfg(target_os = "macos")]
            return ext == "dylib";
        }
        false
    }
}
