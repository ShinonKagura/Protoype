use std::fs::{self, File};
use std::path::PathBuf;
use zstd::stream::{copy_encode, copy_decode};
use crate::plugin_api::base::{Plugin, PluginConfig, PluginError, PluginType};
use crate::plugin_api::compression::CompressionPlugin;
use crate::plugin_api::types::{CompressionOptions, PluginMetadata};
use log::info;
use async_trait::async_trait;
use anyhow::Result;

#[derive(Clone)]
pub struct ZstdPlugin {
    config: PluginConfig,
    metadata: PluginMetadata,
}

impl ZstdPlugin {
    pub fn new() -> Self {
        Self {
            config: PluginConfig {
                name: "zstd".to_string(),
                description: "Zstandard compression plugin".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                plugin_type: PluginType::Compression,
            },
            metadata: PluginMetadata {
                name: "Zstandard".to_string(),
                description: "High-performance compression using Zstandard algorithm".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        }
    }
}

impl Plugin for ZstdPlugin {
    fn get_config(&self) -> &PluginConfig {
        &self.config
    }

    fn initialize(&mut self) -> Result<(), PluginError> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), PluginError> {
        Ok(())
    }
}

#[async_trait]
impl CompressionPlugin for ZstdPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    async fn compress(
        &self,
        files: Vec<PathBuf>,
        output_path: PathBuf,
        options: CompressionOptions,
    ) -> Result<(), PluginError> {
        if files.is_empty() {
            return Err(PluginError::ExecutionError("No files to compress".to_string()));
        }

        // Create parent directory if it doesn't exist
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| PluginError::ExecutionError(format!("Failed to create output directory: {}", e)))?;
        }

        let input_file = File::open(&files[0])
            .map_err(|e| PluginError::ExecutionError(format!("Failed to open input file: {}", e)))?;

        let output_file = File::create(&output_path)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to create output file: {}", e)))?;

        // Use compression level from options or default to 3
        let level = options.level.unwrap_or(3);
        copy_encode(input_file, output_file, level)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to compress file: {}", e)))?;

        info!("Successfully created ZSTD file: {}", output_path.display());
        Ok(())
    }

    async fn decompress(
        &self,
        archive_path: PathBuf,
        output_dir: PathBuf,
        _options: CompressionOptions,
    ) -> Result<Vec<String>, PluginError> {
        if !archive_path.exists() {
            return Err(PluginError::ExecutionError("Archive file does not exist".to_string()));
        }

        fs::create_dir_all(&output_dir)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to create output directory: {}", e)))?;

        let file_stem = archive_path.file_stem()
            .ok_or_else(|| PluginError::ExecutionError("Invalid archive filename".to_string()))?;

        let output_path = output_dir.join(file_stem);
        let input_file = File::open(&archive_path)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to open archive: {}", e)))?;

        let output_file = File::create(&output_path)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to create output file: {}", e)))?;

        copy_decode(input_file, output_file)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to decompress file: {}", e)))?;

        Ok(vec![output_path.to_string_lossy().into_owned()])
    }

    fn supported_extensions(&self) -> Vec<&str> {
        vec!["zst", "zstd"]
    }

    fn default_extension(&self) -> &str {
        "zst"
    }
}
