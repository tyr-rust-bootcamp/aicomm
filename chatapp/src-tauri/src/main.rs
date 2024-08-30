// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use chatapp_lib::app;

fn main() -> Result<()> {
    app()?
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
