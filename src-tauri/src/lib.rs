mod config;
mod storage;
mod types;

use crate::{
    config::Data,
    storage::{add_profile, get_profiles, get_version},
};
use configura::load_config;
use parking_lot::Mutex;

#[derive(Debug)]
pub struct AppState {
    pub debug: bool,
    pub config: Mutex<Data>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(debug: bool) {
    let config: Data = load_config().expect("Failed to load config file");

    tauri::Builder::default()
        .manage(AppState {
            debug,
            config: Mutex::new(config),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            add_profile,
            get_profiles,
            get_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
