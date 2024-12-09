use std::any::Any;
use std::path::PathBuf;
use crate::plugin_api::base::{Plugin, PluginError};
use crate::plugin_api::types::{PluginMetadata, CompressionOptions};
use crate::plugin_api::compression::CompressionPlugin;

/// Template Plugin - Use this as a base for creating new plugins
/// This template follows the core principles:
/// 1. Plugin Independence: No dependencies on core files
/// 2. Clean Architecture: Clear separation of concerns
/// 3. Platform Independence: Cross-platform support
pub struct TemplatePlugin {
    metadata: PluginMetadata,
}

impl TemplatePlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: String::from("template"),
                version: String::from("1.0.0"),
                author: String::from("Smart Transfer Team"),
                description: String::from("Template for creating new plugins"),
            },
        }
    }
}

impl Plugin for TemplatePlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl CompressionPlugin for TemplatePlugin {
    fn compress(
        &self,
        input_files: &[PathBuf],
        output_file: &PathBuf,
        options: &CompressionOptions,
    ) -> Result<(), PluginError> {
        // Implement your compression logic here
        // This is just a placeholder implementation
        Err(PluginError::Other("Not implemented".to_string()))
    }

    fn decompress(
        &self,
        input_file: &PathBuf,
        output_dir: &PathBuf,
        overwrite: bool,
    ) -> Result<(), PluginError> {
        // Implement your decompression logic here
        // This is just a placeholder implementation
        Err(PluginError::Other("Not implemented".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let plugin = TemplatePlugin::new();
        let metadata = plugin.metadata();
        assert_eq!(metadata.name, "template");
        assert_eq!(metadata.version, "1.0.0");
        assert!(!metadata.description.is_empty());
    }

    #[test]
    fn test_compression_not_implemented() {
        let plugin = TemplatePlugin::new();
        let result = plugin.compress(
            &[PathBuf::from("test.txt")],
            &PathBuf::from("test.zip"),
            &CompressionOptions::default(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_decompression_not_implemented() {
        let plugin = TemplatePlugin::new();
        let result = plugin.decompress(
            &PathBuf::from("test.zip"),
            &PathBuf::from("output"),
            true,
        );
        assert!(result.is_err());
    }
}
