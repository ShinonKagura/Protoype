use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;

pub trait PlatformPaths {
    fn get_plugin_dir() -> PathBuf;
    fn get_log_dir() -> PathBuf;
    fn get_config_dir() -> PathBuf;
}

#[cfg(target_os = "windows")]
pub use self::windows::WindowsPaths as PlatformPathsImpl;
#[cfg(target_os = "linux")]
pub use self::linux::LinuxPaths as PlatformPathsImpl;
#[cfg(target_os = "macos")]
pub use self::macos::MacOSPaths as PlatformPathsImpl;

pub fn get_plugin_dir() -> PathBuf {
    PlatformPathsImpl::get_plugin_dir()
}

pub fn get_log_dir() -> PathBuf {
    PlatformPathsImpl::get_log_dir()
}

pub fn get_config_dir() -> PathBuf {
    PlatformPathsImpl::get_config_dir()
}
