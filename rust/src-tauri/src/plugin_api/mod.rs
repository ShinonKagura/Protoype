pub mod base;
pub mod types;
pub mod compression;
pub mod platform;

// Re-export commonly used types
pub use base::{Plugin, PluginFactory};
pub use types::{
    PluginMetadata,
    CompressionOptions,
    PlatformSupport,
    PluginType,
    CompressionMode,
};
pub use compression::CompressionPlugin;
pub use platform::{
    PlatformPaths,
    get_plugin_dir,
    get_log_dir,
    get_config_dir,
};
