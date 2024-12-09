use std::path::{Path, PathBuf};
use std::process::Command;
use crate::plugin_api::base::{Plugin, PluginConfig, PluginError, PluginType};
use crate::plugin_api::compression::{CompressionMode, CompressionOptions, CompressionPlugin};

pub struct SevenZipPlugin {
    config: PluginConfig,
}

impl SevenZipPlugin {
    pub fn new() -> Self {
        SevenZipPlugin {
            config: PluginConfig {
                name: "7z".to_string(),
                description: "7-Zip compression plugin".to_string(),
                version: "1.0.0".to_string(),
                plugin_type: PluginType::Compression,
            },
        }
    }
}

impl Plugin for SevenZipPlugin {
    fn get_config(&self) -> &PluginConfig {
        &self.config
    }

    fn initialize(&mut self) -> Result<(), PluginError> {
        // Check if 7z is installed
        let output = Command::new("7z")
            .arg("--help")
            .output()
            .map_err(|e| PluginError::ExecutionError(format!("7-Zip is not installed: {}", e)))?;

        if !output.status.success() {
            return Err(PluginError::ExecutionError("7-Zip is not installed".to_string()));
        }

        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), PluginError> {
        Ok(())
    }
}

impl CompressionPlugin for SevenZipPlugin {
    fn compress(
        &self,
        files: &[PathBuf],
        output: PathBuf,
        options: &CompressionOptions,
    ) -> Result<(), PluginError> {
        let mut cmd = Command::new("7z");
        cmd.arg("a")  // Add to archive
           .arg("-t7z")  // 7z format
           .arg("-mx=5")  // Normal compression
           .arg(&output);

        // Add all files to the command
        for file in files {
            cmd.arg(file);
        }

        let output = cmd.output()
            .map_err(|e| PluginError::ExecutionError(format!("Failed to execute 7z: {}", e)))?;

        if !output.status.success() {
            return Err(PluginError::ExecutionError(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }

        Ok(())
    }

    fn decompress(
        &self,
        archive: PathBuf,
        output_dir: PathBuf,
    ) -> Result<(), PluginError> {
        let output = Command::new("7z")
            .arg("x")  // Extract with full paths
            .arg(&archive)
            .arg(format!("-o{}", output_dir.display()))  // Output directory
            .arg("-y")  // Yes to all queries
            .output()
            .map_err(|e| PluginError::ExecutionError(format!("Failed to execute 7z: {}", e)))?;

        if !output.status.success() {
            return Err(PluginError::ExecutionError(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }

        Ok(())
    }
}
