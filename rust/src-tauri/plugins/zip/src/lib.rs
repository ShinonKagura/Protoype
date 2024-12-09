use std::fs::{self, File};
use std::io::{Read, Write, Seek};
use std::path::PathBuf;
use zip::{ZipWriter, write::FileOptions};
use walkdir::WalkDir;
use crate::plugin_api::base::{Plugin, PluginConfig, PluginError, PluginType};
use crate::plugin_api::compression::{CompressionMode, CompressionOptions, CompressionPlugin};

pub struct ZipPlugin {
    config: PluginConfig,
}

impl ZipPlugin {
    pub fn new() -> Self {
        ZipPlugin {
            config: PluginConfig {
                name: "zip".to_string(),
                description: "ZIP compression plugin".to_string(),
                version: "1.0.0".to_string(),
                plugin_type: PluginType::Compression,
            },
        }
    }
}

impl Plugin for ZipPlugin {
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

impl CompressionPlugin for ZipPlugin {
    fn compress(
        &self,
        files: &[PathBuf],
        output: PathBuf,
        options: &CompressionOptions,
    ) -> Result<(), PluginError> {
        let file = File::create(&output)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to create ZIP file: {}", e)))?;

        let mut zip = ZipWriter::new(file);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        for path in files {
            if path.is_file() {
                let file_name = path.file_name()
                    .ok_or_else(|| PluginError::ExecutionError("Invalid file name".to_string()))?
                    .to_string_lossy();

                zip.start_file(file_name, options)
                    .map_err(|e| PluginError::ExecutionError(format!("Failed to add file to ZIP: {}", e)))?;

                let mut f = File::open(path)
                    .map_err(|e| PluginError::ExecutionError(format!("Failed to open file: {}", e)))?;
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer)
                    .map_err(|e| PluginError::ExecutionError(format!("Failed to read file: {}", e)))?;
                zip.write_all(&buffer)
                    .map_err(|e| PluginError::ExecutionError(format!("Failed to write file to ZIP: {}", e)))?;
            } else if path.is_dir() {
                for entry in WalkDir::new(path) {
                    let entry = entry.map_err(|e| PluginError::ExecutionError(format!("Failed to read directory: {}", e)))?;
                    let entry_path = entry.path();
                    
                    if entry_path.is_file() {
                        let relative_path = entry_path.strip_prefix(path)
                            .map_err(|e| PluginError::ExecutionError(format!("Failed to get relative path: {}", e)))?;
                        
                        zip.start_file(relative_path.to_string_lossy(), options)
                            .map_err(|e| PluginError::ExecutionError(format!("Failed to add file to ZIP: {}", e)))?;

                        let mut f = File::open(entry_path)
                            .map_err(|e| PluginError::ExecutionError(format!("Failed to open file: {}", e)))?;
                        let mut buffer = Vec::new();
                        f.read_to_end(&mut buffer)
                            .map_err(|e| PluginError::ExecutionError(format!("Failed to read file: {}", e)))?;
                        zip.write_all(&buffer)
                            .map_err(|e| PluginError::ExecutionError(format!("Failed to write file to ZIP: {}", e)))?;
                    }
                }
            }
        }

        zip.finish()
            .map_err(|e| PluginError::ExecutionError(format!("Failed to finalize ZIP file: {}", e)))?;

        Ok(())
    }

    fn decompress(
        &self,
        archive: PathBuf,
        output_dir: PathBuf,
    ) -> Result<(), PluginError> {
        let file = File::open(&archive)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to open ZIP file: {}", e)))?;

        let mut archive = zip::ZipArchive::new(file)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to read ZIP archive: {}", e)))?;

        fs::create_dir_all(&output_dir)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to create output directory: {}", e)))?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| PluginError::ExecutionError(format!("Failed to read file from ZIP: {}", e)))?;
            
            let outpath = output_dir.join(file.name());

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)
                    .map_err(|e| PluginError::ExecutionError(format!("Failed to create directory: {}", e)))?;
            } else {
                if let Some(p) = outpath.parent() {
                    fs::create_dir_all(p)
                        .map_err(|e| PluginError::ExecutionError(format!("Failed to create parent directory: {}", e)))?;
                }
                let mut outfile = File::create(&outpath)
                    .map_err(|e| PluginError::ExecutionError(format!("Failed to create output file: {}", e)))?;
                std::io::copy(&mut file, &mut outfile)
                    .map_err(|e| PluginError::ExecutionError(format!("Failed to write output file: {}", e)))?;
            }
        }

        Ok(())
    }
}
