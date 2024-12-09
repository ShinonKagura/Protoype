pub mod core;
pub mod plugin_api;
pub mod plugins;

// Re-export commonly used items
pub use core::{PluginManager, init_logging, get_logs_directory, cleanup_old_logs};
pub use plugin_api::compression::CompressionPlugin;
pub use plugin_api::types::{CompressionMode, CompressionOptions, PluginMetadata, PluginType};
