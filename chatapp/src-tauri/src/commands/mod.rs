use std::sync::Arc;

use tauri::{AppHandle, Manager};
use tracing::info;

use crate::{config::AppConfig, utils::app_dir, AppState};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub(crate) fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub(crate) fn get_app_dir() -> String {
    app_dir().display().to_string()
}

#[tauri::command]
pub(crate) fn get_config(handle: AppHandle) -> Arc<AppConfig> {
    let conf = handle.state::<AppState>().config.load().clone();
    info!("Config: {:?}", conf);
    conf
}
