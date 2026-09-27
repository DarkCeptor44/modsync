#![allow(clippy::needless_pass_by_value)]

use crate::{
    AppState,
    sync::{sync, types::SyncOutcome},
    types::{Profile, ProfileInput},
};
use tauri::{AppHandle, Emitter, State};
use tokio::{spawn, sync::mpsc};

#[tauri::command]
pub fn add_profile(state: State<'_, AppState>, profile: ProfileInput) -> Result<String, String> {
    let mut config = state.config.lock();
    if config
        .profiles
        .iter()
        .any(|(_, p)| p.name.to_lowercase() == profile.name.to_lowercase())
    {
        return Err("Profile name already exists".to_string());
    }

    let msg = format!("added profile: profile={profile:?}");
    let id = config.add_profile(profile).map_err(|e| e.to_string())?;

    if state.debug {
        println!("{msg}");
    }

    Ok(id)
}

#[tauri::command]
pub fn edit_profile(state: State<'_, AppState>, profile: Profile) -> Result<String, String> {
    if profile.id.trim().is_empty() {
        return Err("Profile ID cannot be empty".to_string());
    }

    let mut config = state.config.lock();
    if !config.profiles.contains_key(&profile.id) {
        return Err("Profile not found".to_string());
    }

    let msg = format!("updated profile: profile={profile:?}");
    let new_id = config.update_profile(profile).map_err(|e| e.to_string())?;

    if state.debug {
        println!("{msg}");
    }

    Ok(new_id)
}

#[tauri::command]
pub fn get_profiles(state: State<'_, AppState>) -> Vec<Profile> {
    if state.debug {
        println!("getting profiles");
    }

    let config = state.config.lock();
    let profiles: Vec<Profile> = config
        .profiles
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()).into())
        .collect();

    profiles
}

#[tauri::command]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub async fn sync_profile(
    app: AppHandle,
    state: State<'_, AppState>,
    profile: Profile,
) -> Result<(), String> {
    if state.debug {
        println!("syncing profile: profile={profile:?}");
    }

    let (tx, mut rx) = mpsc::unbounded_channel::<SyncOutcome>();
    let app_handle = app.clone();
    let listener = spawn(async move {
        while let Some(outcome) = rx.recv().await {
            let _ = app_handle.emit("sync-progress", outcome);
        }
    });

    let result = sync(&profile, tx, num_cpus::get(), true)
        .await
        .map_err(|e| e.to_string());
    let _ = listener.await;

    result
}
