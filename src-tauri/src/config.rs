use crate::types::{Profile, ProfileInput};
use anyhow::{Result, anyhow};
use configura::{Config, formats::JsonFormat};
use heck::ToKebabCase;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const CONFIG_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Data {
    pub profiles: HashMap<String, ProfileInput>,
}

impl Config for Data {
    type FormatType = JsonFormat;
    type FormatContext = ();

    fn config_path_and_filename(_home_dir: &std::path::Path) -> (Option<std::path::PathBuf>, &str) {
        (None, CONFIG_NAME)
    }
}

impl Data {
    pub fn add_profile(&mut self, profile: ProfileInput) -> Result<()> {
        if profile.name.trim().is_empty() {
            return Err(anyhow!("Profile name cannot be empty"));
        }

        if profile.source.is_empty() {
            return Err(anyhow!("Profile source cannot be empty"));
        }

        if profile.destination.is_empty() {
            return Err(anyhow!("Profile destination cannot be empty"));
        }

        let mut cleaned_exclusions = Vec::new();
        for exclusion in profile.exclusions {
            if exclusion.is_empty() {
                continue;
            }

            cleaned_exclusions.push(exclusion);
        }

        let id = profile.name.trim().to_kebab_case();
        self.profiles.insert(
            id,
            ProfileInput {
                name: profile.name,
                source: profile.source,
                destination: profile.destination,
                exclusions: cleaned_exclusions,
            },
        );
        self.save()?;

        Ok(())
    }

    pub fn update_profile(&mut self, mut profile: Profile) -> Result<String> {
        if profile.id.trim().is_empty() {
            return Err(anyhow!("Profile ID cannot be empty"));
        }

        let name = profile.name.trim();
        if name.is_empty() {
            return Err(anyhow!("Profile name cannot be empty"));
        }

        if profile.source.is_empty() {
            return Err(anyhow!("Profile source cannot be empty"));
        }

        if profile.destination.is_empty() {
            return Err(anyhow!("Profile destination cannot be empty"));
        }

        profile.exclusions.retain(|e| !e.is_empty());

        let new_id = name.to_kebab_case();
        if profile.id != new_id {
            self.profiles.remove(&profile.id);
        }

        self.profiles.insert(new_id.clone(), profile.into());
        self.save()?;

        Ok(new_id)
    }
}
