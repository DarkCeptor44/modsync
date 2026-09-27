use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub sync_exclusions: Vec<PathBuf>,
    pub delete_exclusions: Vec<PathBuf>,
}

impl From<(String, ProfileInput)> for Profile {
    fn from(value: (String, ProfileInput)) -> Self {
        Self {
            id: value.0,
            name: value.1.name,
            source: value.1.source,
            destination: value.1.destination,
            sync_exclusions: value.1.sync_exclusions,
            delete_exclusions: value.1.delete_exclusions,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInput {
    pub name: String,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub sync_exclusions: Vec<PathBuf>,
    pub delete_exclusions: Vec<PathBuf>,
}

impl From<Profile> for ProfileInput {
    fn from(value: Profile) -> Self {
        Self {
            name: value.name,
            source: value.source,
            destination: value.destination,
            sync_exclusions: value.sync_exclusions,
            delete_exclusions: value.delete_exclusions,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub jobs: usize,
}
