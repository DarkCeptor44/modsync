#![forbid(unsafe_code)]
#![warn(clippy::pedantic, missing_debug_implementations)]

mod config;
mod storage;
pub mod sync;
mod types;

use crate::{
    config::Data,
    storage::{
        add_profile, delete_profile, edit_profile, get_profiles, get_settings, get_version,
        save_settings, sync_profile,
    },
};
use anyhow::{Context, Result};
use configura::load_config;
use parking_lot::Mutex;
use tauri::Builder;

#[derive(Debug)]
pub(crate) struct AppState {
    pub debug: bool,
    pub config: Mutex<Data>,
}

/// Entry point of the application
///
/// ## Errors
///
/// Returns an error if the application fails to run or if the config file fails to load
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(debug: bool) -> Result<()> {
    let config: Data = load_config().context("Failed to load config file")?;

    Builder::default()
        .manage(AppState {
            debug,
            config: Mutex::new(config),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            add_profile,
            delete_profile,
            edit_profile,
            get_settings,
            get_profiles,
            get_version,
            save_settings,
            sync_profile
        ])
        .run(tauri::generate_context!())
        .context("error while running tauri application")
}
