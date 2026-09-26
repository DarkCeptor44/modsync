use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncOutcome {
    pub entry: PathBuf,
    pub action: SyncAction,
}

impl SyncOutcome {
    #[must_use]
    pub fn copied(entry: PathBuf, bytes: u64) -> Self {
        Self {
            entry,
            action: SyncAction::Copied { bytes },
        }
    }

    #[must_use]
    pub fn removed(entry: PathBuf, bytes: u64) -> Self {
        Self {
            entry,
            action: SyncAction::Removed { bytes },
        }
    }

    #[must_use]
    pub fn skipped(entry: PathBuf) -> Self {
        Self {
            entry,
            action: SyncAction::Skipped,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncAction {
    Copied { bytes: u64 },
    Removed { bytes: u64 },
    Skipped,
    Failed { error: String },
}
