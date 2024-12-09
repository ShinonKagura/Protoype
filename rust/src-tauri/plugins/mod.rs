use crate::plugin_api::base::{Plugin, PluginConfig, PluginError};
use crate::plugin_api::compression::{CompressionOptions, CompressionPlugin};
use std::path::PathBuf;

// Import plugins from their dedicated directories
use crate::plugins::zip::ZipPlugin;
use crate::plugins::sevenz::SevenZipPlugin;
use crate::plugins::zstd::ZstdPlugin;

#[derive(Clone)]
pub enum PluginInstance {
    Zip(ZipPlugin),
    SevenZip(SevenZipPlugin),
    Zstd(ZstdPlugin),
}

impl Plugin for PluginInstance {
    fn get_config(&self) -> &PluginConfig {
        match self {
            PluginInstance::Zip(p) => p.get_config(),
            PluginInstance::SevenZip(p) => p.get_config(),
            PluginInstance::Zstd(p) => p.get_config(),
        }
    }

    fn initialize(&mut self) -> Result<(), PluginError> {
        match self {
            PluginInstance::Zip(p) => p.initialize(),
            PluginInstance::SevenZip(p) => p.initialize(),
            PluginInstance::Zstd(p) => p.initialize(),
        }
    }

    fn shutdown(&mut self) -> Result<(), PluginError> {
        match self {
            PluginInstance::Zip(p) => p.shutdown(),
            PluginInstance::SevenZip(p) => p.shutdown(),
            PluginInstance::Zstd(p) => p.shutdown(),
        }
    }
}

impl CompressionPlugin for PluginInstance {
    fn compress(
        &self,
        files: &[PathBuf],
        output: PathBuf,
        options: &CompressionOptions,
    ) -> Result<(), PluginError> {
        match self {
            PluginInstance::Zip(p) => p.compress(files, output, options),
            PluginInstance::SevenZip(p) => p.compress(files, output, options),
            PluginInstance::Zstd(p) => p.compress(files, output, options),
        }
    }

    fn decompress(
        &self,
        archive: PathBuf,
        output_dir: PathBuf,
        overwrite: bool,
    ) -> Result<Vec<String>, PluginError> {
        match self {
            PluginInstance::Zip(p) => p.decompress(archive.clone(), output_dir.clone(), overwrite),
            PluginInstance::SevenZip(p) => p.decompress(archive.clone(), output_dir.clone(), overwrite),
            PluginInstance::Zstd(p) => p.decompress(archive.clone(), output_dir.clone(), overwrite),
        }
    }
}
