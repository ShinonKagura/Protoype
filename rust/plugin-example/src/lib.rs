use anyhow::Result;
use async_trait::async_trait;
use smart_transfer_plugin_api::{
    base::{Plugin, PluginConfig, PluginType},
    compression::{CompressionPlugin, CompressionOptions},
    types::PluginMetadata,
    declare_plugin,
};

#[derive(Clone)]
pub struct ExamplePlugin {
    config: PluginConfig,
    metadata: PluginMetadata,
}

impl ExamplePlugin {
    pub fn new() -> Self {
        Self {
            config: PluginConfig {
                name: "example".to_string(),
                description: "Example plugin".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                plugin_type: PluginType::Compression,
            },
            metadata: PluginMetadata {
                name: "Example Plugin".to_string(),
                description: "An example plugin implementation".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                author: "Your Name".to_string(),
                plugin_type: PluginType::Compression,
            },
        }
    }
}

impl Plugin for ExamplePlugin {
    fn get_config(&self) -> &PluginConfig {
        &self.config
    }

    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl CompressionPlugin for ExamplePlugin {
    async fn compress(
        &self,
        _files: Vec<std::path::PathBuf>,
        _output_path: std::path::PathBuf,
        _options: CompressionOptions,
    ) -> Result<()> {
        Ok(())
    }

    async fn decompress(
        &self,
        _archive_path: std::path::PathBuf,
        _output_dir: std::path::PathBuf,
        _options: CompressionOptions,
    ) -> Result<()> {
        Ok(())
    }

    fn supported_extensions(&self) -> Vec<&str> {
        vec!["example"]
    }

    fn default_extension(&self) -> &str {
        "example"
    }
}

declare_plugin!(ExamplePlugin, ExamplePlugin::new);
