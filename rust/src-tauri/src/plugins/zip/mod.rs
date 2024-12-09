use std::any::Any;
use std::borrow::Cow;
use std::path::PathBuf;
use anyhow::Result;
use crate::plugin_api::base::{Plugin, PluginError, PluginConfig};
use crate::plugin_api::compression::CompressionPlugin;
use crate::plugin_api::types::{PluginMetadata, PlatformSupport, PluginType, CompressionOptions};
use zip::write::FileOptions;
use zip::ZipArchive;
use std::fs::File;
use std::io::{Read, Write};

pub struct ZipPlugin {
    config: PluginConfig,
    metadata: PluginMetadata,
}

impl ZipPlugin {
    pub fn new() -> Self {
        ZipPlugin {
            config: PluginConfig {
                name: String::from("ZIP Plugin"),
                description: String::from("Handles ZIP file compression and decompression"),
                version: String::from("1.0.0"),
                plugin_type: PluginType::Compression,
            },
            metadata: PluginMetadata {
                name: String::from("ZIP Plugin"),
                description: String::from("Handles ZIP file compression and decompression"),
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

impl Plugin for ZipPlugin {
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

impl CompressionPlugin for ZipPlugin {
    fn compress(
        &self,
        input_files: &[PathBuf],
        output_file: &PathBuf,
        _options: &CompressionOptions,
    ) -> Result<(), PluginError> {
        // Create output file
        let file = File::create(output_file)?;
        let mut zip = zip::ZipWriter::new(file);
        let options = FileOptions::default();

        // Process each input file
        for input_file in input_files {
            let mut file = File::open(input_file)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            let name = input_file
                .file_name()
                .ok_or_else(|| PluginError::InvalidInput("Invalid file name".to_string()))?
                .to_string_lossy();

            zip.start_file(name.as_ref(), options).map_err(|e| PluginError::Other(e.to_string()))?;
            zip.write_all(&buffer).map_err(|e| PluginError::Other(e.to_string()))?;
        }

        // Finish writing zip file
        zip.finish().map_err(|e| PluginError::Other(e.to_string()))?;
        Ok(())
    }

    fn decompress(
        &self,
        archive_file: &PathBuf,
        output_dir: &PathBuf,
        _overwrite: bool,
    ) -> Result<(), PluginError> {
        let file = File::open(archive_file)?;
        let mut archive = ZipArchive::new(file).map_err(|e| PluginError::Other(e.to_string()))?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| PluginError::Other(e.to_string()))?;
            let outpath = match file.enclosed_name() {
                Some(path) => output_dir.join(path),
                None => continue,
            };

            if let Some(p) = outpath.parent() {
                std::fs::create_dir_all(p)?;
            }

            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_zip_plugin() {
        let plugin = ZipPlugin::new();
        assert_eq!(plugin.get_config().name, "ZIP Plugin");
    }
}
