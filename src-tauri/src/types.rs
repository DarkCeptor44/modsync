use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub exclusions: Vec<PathBuf>,
}

impl From<(String, ProfileInput)> for Profile {
    fn from(value: (String, ProfileInput)) -> Self {
        Self {
            id: value.0,
            name: value.1.name,
            source: value.1.source,
            destination: value.1.destination,
            exclusions: value.1.exclusions,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfileInput {
    pub name: String,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub exclusions: Vec<PathBuf>,
}

impl From<Profile> for ProfileInput {
    fn from(value: Profile) -> Self {
        Self {
            name: value.name,
            source: value.source,
            destination: value.destination,
            exclusions: value.exclusions,
        }
    }
}
