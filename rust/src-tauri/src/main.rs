// Prevents additional console window on Windows in release
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;
use tauri::State;
use smart_transfer::plugin_api::types::{CompressionOptions, PluginMetadata, CompressionMode};
use smart_transfer::plugin_api::compression::CompressionPlugin;
use smart_transfer::core::plugin_manager::PluginManager;

struct AppState {
    plugin_manager: PluginManager,
}

#[tauri::command]
fn compress_files(
    plugin_name: String,
    input_files: Vec<String>,
    output_file: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let input_paths: Vec<PathBuf> = input_files.into_iter().map(PathBuf::from).collect();
    let output_path = PathBuf::from(output_file);

    let plugin = state.plugin_manager
        .get_plugin(&plugin_name)
        .ok_or_else(|| format!("Plugin '{}' not found", plugin_name))?;

    if let Some(compression_plugin) = plugin.as_any().downcast_ref::<Box<dyn CompressionPlugin>>() {
        compression_plugin
            .compress(
                &input_paths,
                &output_path,
                &CompressionOptions {
                    mode: CompressionMode::Normal,
                    password: None,
                    split_size: None,
                    extra_args: Default::default(),
                },
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err(format!("Plugin '{}' is not a compression plugin", plugin_name))
    }
}

#[tauri::command]
fn decompress_file(
    plugin_name: String,
    input_file: String,
    output_dir: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let input_path = PathBuf::from(input_file);
    let output_path = PathBuf::from(output_dir);

    let plugin = state.plugin_manager
        .get_plugin(&plugin_name)
        .ok_or_else(|| format!("Plugin '{}' not found", plugin_name))?;

    if let Some(compression_plugin) = plugin.as_any().downcast_ref::<Box<dyn CompressionPlugin>>() {
        compression_plugin
            .decompress(&input_path, &output_path, true)
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err(format!("Plugin '{}' is not a compression plugin", plugin_name))
    }
}

#[tauri::command]
fn list_plugins(state: State<'_, AppState>) -> Vec<PluginMetadata> {
    state.plugin_manager.list_plugins()
}

fn main() {
    let mut plugin_manager = PluginManager::new();
    
    if let Err(e) = plugin_manager.register_default_plugins() {
        eprintln!("Error loading plugins: {}", e);
        return;
    }

    tauri::Builder::default()
        .manage(AppState { plugin_manager })
        .invoke_handler(tauri::generate_handler![
            compress_files,
            decompress_file,
            list_plugins
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
