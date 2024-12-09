use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub plugin_type: PluginType,
    pub platform_support: PlatformSupport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginType {
    Compression,
    Transfer,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformSupport {
    pub windows: bool,
    pub linux: bool,
    pub macos: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionMode {
    Fast,
    Normal,
    Best,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionOptions {
    pub mode: CompressionMode,
    pub password: Option<String>,
    pub split_size: Option<u64>,
    pub extra_args: HashMap<String, String>,
}
