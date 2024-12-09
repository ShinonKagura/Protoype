use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginConfig {
    name: String,
    version: String,
    author: String,
    description: String,
}

#[derive(Debug)]
pub struct Plugin {
    config: PluginConfig,
    initialized: bool,
}

impl Plugin {
    #[no_mangle]
    pub extern "C" fn create() -> *mut Plugin {
        let config = PluginConfig {
            name: "Plugin Template".to_string(),
            version: "0.1.0".to_string(),
            author: "Your Name".to_string(),
            description: "A template for Smart Transfer plugins".to_string(),
        };

        let plugin = Box::new(Plugin {
            config,
            initialized: false,
        });

        Box::into_raw(plugin)
    }

    #[no_mangle]
    pub extern "C" fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        if self.initialized {
            return Ok(());
        }
        // Add initialization logic here
        self.initialized = true;
        Ok(())
    }

    #[no_mangle]
    pub extern "C" fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.initialized {
            return Ok(());
        }
        // Add cleanup logic here
        self.initialized = false;
        Ok(())
    }

    #[no_mangle]
    pub extern "C" fn get_config(&self) -> *const PluginConfig {
        &self.config
    }
}

#[no_mangle]
pub extern "C" fn destroy(plugin: *mut Plugin) {
    unsafe {
        if !plugin.is_null() {
            drop(Box::from_raw(plugin));
        }
    }
}
