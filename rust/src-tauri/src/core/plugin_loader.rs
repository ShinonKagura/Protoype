use anyhow::Result;
use crate::plugins::{PluginInstance, PluginLoader};
use std::path::PathBuf;

pub async fn load_plugins(plugin_dir: PathBuf) -> Result<Vec<PluginInstance>> {
    PluginLoader::discover_plugins(plugin_dir)
}
