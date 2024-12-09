use std::any::Any;
use std::io::Cursor;
use std::path::PathBuf;
use anyhow::Result;
use crate::plugin_api::base::{Plugin, PluginError, PluginConfig};
use crate::plugin_api::compression::CompressionPlugin;
use crate::plugin_api::types::{CompressionOptions, PluginMetadata, PlatformSupport, PluginType};
use sevenz_rust;
use tempfile;

pub struct SevenZipPlugin {
    config: PluginConfig,
    metadata: PluginMetadata,
}

impl SevenZipPlugin {
    pub fn new() -> Self {
        SevenZipPlugin {
            config: PluginConfig {
                name: String::from("7-Zip Plugin"),
                description: String::from("Handles 7z file compression and decompression"),
                version: String::from("1.0.0"),
                plugin_type: PluginType::Compression,
            },
            metadata: PluginMetadata {
                name: String::from("7-Zip Plugin"),
                description: String::from("Handles 7z file compression and decompression"),
                version: String::from("1.0.0"),
                author: String::from("Smart Transfer Team"),
                platform_support: PlatformSupport {
                    windows: true,
                    linux: true,
                    macos: true,
                },
                plugin_type: PluginType::Compression,
            },
        }
    }
}

impl Plugin for SevenZipPlugin {
    fn get_config(&self) -> &PluginConfig {
        &self.config
    }

    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn initialize(&mut self) -> Result<(), PluginError> {
        Ok(())
    }

    fn cleanup(&mut self) -> Result<(), PluginError> {
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl CompressionPlugin for SevenZipPlugin {
    fn compress(
        &self,
        input_files: &[PathBuf],
        output_file: &PathBuf,
        _options: &CompressionOptions,
    ) -> Result<(), PluginError> {
        // Create a temporary directory to store files
        let temp_dir = tempfile::tempdir().map_err(|e| PluginError::Other(e.to_string()))?;
        
        // Copy files to temp directory to maintain relative paths
        for input_file in input_files {
            let file_name = input_file.file_name()
                .ok_or_else(|| PluginError::InvalidInput("Invalid file name".to_string()))?;
            let temp_path = temp_dir.path().join(file_name);
            std::fs::copy(input_file, &temp_path).map_err(PluginError::from)?;
        }

        // Compress the directory
        sevenz_rust::compress_to_path(temp_dir.path(), output_file)
            .map_err(|e| PluginError::Other(e.to_string()))?;

        Ok(())
    }

    fn decompress(
        &self,
        archive_file: &PathBuf,
        output_dir: &PathBuf,
        _overwrite: bool,
    ) -> Result<(), PluginError> {
        // Create output directory
        std::fs::create_dir_all(output_dir)?;

        // Read archive file
        let archive_data = std::fs::read(archive_file)?;

        // Decompress using sevenz-rust
        sevenz_rust::decompress(
            &mut Cursor::new(archive_data),
            output_dir,
        ).map_err(|e| PluginError::Other(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_sevenz_plugin() {
        let plugin = SevenZipPlugin::new();
        assert_eq!(plugin.get_config().name, "7-Zip Plugin");
    }
}
