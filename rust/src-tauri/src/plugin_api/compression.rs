use super::base::{Plugin, PluginError};
use super::types::CompressionOptions;
use std::path::PathBuf;

pub trait CompressionPlugin: Plugin {
    fn compress(
        &self,
        input_files: &[PathBuf],
        output_file: &PathBuf,
        options: &CompressionOptions,
    ) -> Result<(), PluginError>;

    fn decompress(
        &self,
        archive_file: &PathBuf,
        output_dir: &PathBuf,
        overwrite: bool,
    ) -> Result<(), PluginError>;
}
