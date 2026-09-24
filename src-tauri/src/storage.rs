use crate::{
    AppState,
    types::{Profile, ProfileInput},
};
use tauri::State;

#[tauri::command]
pub fn add_profile(state: State<'_, AppState>, profile: ProfileInput) -> Result<(), String> {
    let msg = format!("added profile: {profile:?}");

    let mut config = state.config.lock();
    if config
        .profiles
        .iter()
        .any(|(_, p)| p.name.to_lowercase() == profile.name.to_lowercase())
    {
        return Err("Profile name already exists".to_string());
    }

    config.add_profile(profile).map_err(|e| e.to_string())?;

    if state.debug {
        println!("{msg}");
    }

    Ok(())
}

#[tauri::command]
pub fn get_profiles(state: State<'_, AppState>) -> Result<Vec<Profile>, String> {
    if state.debug {
        println!("getting profiles");
    }

    let config = state.config.lock();
    let profiles: Vec<Profile> = config
        .profiles
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()).into())
        .collect();

    Ok(profiles)
}

#[tauri::command]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
