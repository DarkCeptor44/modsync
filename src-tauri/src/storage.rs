// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(clippy::needless_pass_by_value)]

use crate::{
    AppState,
    sync::{sync, types::SyncOutcome},
    types::{Profile, ProfileInput, Settings},
};
use configura::Config;
use num_traits::ToPrimitive;
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
pub fn delete_profile(state: State<'_, AppState>, id: String) -> Result<(), String> {
    if id.trim().is_empty() {
        return Err("Profile ID cannot be empty".to_string());
    }

    let mut config = state.config.lock();
    let Some(removed_profile) = config.profiles.remove(&id) else {
        return Err("Profile not found".to_string());
    };

    config.save().map_err(|e| e.to_string())?;
    if state.debug {
        println!("deleted profile: id={id} removed_profile={removed_profile:?}");
    }

    Ok(())
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
pub fn get_settings(state: State<'_, AppState>) -> Settings {
    if state.debug {
        println!("getting concurrency limit");
    }

    let config = state.config.lock();
    let saved_limit = config.concurrency_limit;

    Settings {
        jobs: saved_limit
            .unwrap_or_else(|| {
                let cpus = num_cpus::get();

                (cpus.to_f64().unwrap_or(1.0) * 0.75)
                    .ceil()
                    .to_usize()
                    .unwrap_or(1)
            })
            .max(1),
    }
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
pub fn save_settings(state: State<'_, AppState>, settings: Settings) -> Result<(), String> {
    if state.debug {
        println!("saving settings: settings={settings:?}");
    }

    let mut config = state.config.lock();
    config.concurrency_limit = Some(settings.jobs);
    config.save().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sync_profile(
    app: AppHandle,
    state: State<'_, AppState>,
    profile: Profile,
    dry_run: bool,
    settings: Settings,
) -> Result<(), String> {
    if state.debug {
        println!("syncing profile: profile={profile:?} dry_run={dry_run}");
    }

    let (tx, mut rx) = mpsc::unbounded_channel::<SyncOutcome>();
    let app_handle = app.clone();
    let listener = spawn(async move {
        while let Some(outcome) = rx.recv().await {
            let _ = app_handle.emit("sync-progress", outcome);
        }
    });

    let result = sync(&profile, tx, settings, dry_run)
        .await
        .map_err(|e| e.to_string());
    let _ = listener.await;

    result
}
