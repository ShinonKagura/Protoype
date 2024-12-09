use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use zstd::stream::{encode_all, decode_all};
use crate::plugin_api::base::{Plugin, PluginConfig, PluginError, PluginType};
use crate::plugin_api::compression::{CompressionMode, CompressionOptions, CompressionPlugin};

pub struct ZstdPlugin {
    config: PluginConfig,
}

impl ZstdPlugin {
    pub fn new() -> Self {
        ZstdPlugin {
            config: PluginConfig {
                name: "zstd".to_string(),
                description: "Zstandard compression plugin".to_string(),
                version: "1.0.0".to_string(),
                plugin_type: PluginType::Compression,
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

impl CompressionPlugin for ZstdPlugin {
    fn compress(
        &self,
        files: &[PathBuf],
        output: PathBuf,
        options: &CompressionOptions,
    ) -> Result<(), PluginError> {
        // For now, we'll only handle single file compression
        // TODO: Implement multi-file archive support
        if files.len() != 1 {
            return Err(PluginError::ExecutionError(
                "Zstd plugin currently only supports single file compression".to_string()
            ));
        }

        let input_path = &files[0];
        let mut input_file = File::open(input_path)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to open input file: {}", e)))?;

        let mut output_file = File::create(&output)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to create output file: {}", e)))?;

        let mut buffer = Vec::new();
        input_file.read_to_end(&mut buffer)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to read input file: {}", e)))?;

        let compressed = encode_all(&buffer[..], 3)  // Using compression level 3
            .map_err(|e| PluginError::ExecutionError(format!("Failed to compress data: {}", e)))?;

        output_file.write_all(&compressed)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to write compressed data: {}", e)))?;

        Ok(())
    }

    fn decompress(
        &self,
        archive: PathBuf,
        output_dir: PathBuf,
    ) -> Result<(), PluginError> {
        let mut input_file = File::open(&archive)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to open archive: {}", e)))?;

        let mut buffer = Vec::new();
        input_file.read_to_end(&mut buffer)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to read archive: {}", e)))?;

        let decompressed = decode_all(&buffer[..])
            .map_err(|e| PluginError::ExecutionError(format!("Failed to decompress data: {}", e)))?;

        // Create output directory if it doesn't exist
        fs::create_dir_all(&output_dir)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to create output directory: {}", e)))?;

        // Use the archive name without the extension as the output file name
        let output_file_name = archive
            .file_stem()
            .ok_or_else(|| PluginError::ExecutionError("Invalid archive name".to_string()))?;
        
        let output_path = output_dir.join(output_file_name);
        let mut output_file = File::create(&output_path)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to create output file: {}", e)))?;

        output_file.write_all(&decompressed)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to write decompressed data: {}", e)))?;

        Ok(())
    }
}
