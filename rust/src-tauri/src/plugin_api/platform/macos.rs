use std::path::PathBuf;
use directories::BaseDirs;

pub struct MacOSPaths;

impl super::PlatformPaths for MacOSPaths {
    fn get_plugin_dir() -> PathBuf {
        if cfg!(debug_assertions) {
            PathBuf::from("plugins")
        } else if let Some(base_dirs) = BaseDirs::new() {
            base_dirs.data_dir().join("smart_transfer").join("plugins")
        } else {
            PathBuf::from("plugins")
        }
    }

    fn get_log_dir() -> PathBuf {
        if cfg!(debug_assertions) {
            PathBuf::from("logs")
        } else if let Some(base_dirs) = BaseDirs::new() {
            base_dirs.data_dir().join("smart_transfer").join("logs")
        } else {
            PathBuf::from("logs")
        }
    }

    fn get_config_dir() -> PathBuf {
        if cfg!(debug_assertions) {
            PathBuf::from("config")
        } else if let Some(base_dirs) = BaseDirs::new() {
            base_dirs.config_dir().join("smart_transfer")
        } else {
            PathBuf::from("config")
        }
    }
}
