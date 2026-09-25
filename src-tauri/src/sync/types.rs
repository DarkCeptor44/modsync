use crate::sync::utils::human_bytes;
use std::{fmt::Display, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncOutcome {
    pub entry: PathBuf,
    pub action: SyncAction,
}

impl SyncOutcome {
    pub fn copied(entry: PathBuf, bytes: u64) -> Self {
        Self {
            entry,
            action: SyncAction::Copied { bytes },
        }
    }

    pub fn removed(entry: PathBuf, bytes: u64) -> Self {
        Self {
            entry,
            action: SyncAction::Removed { bytes },
        }
    }

    pub fn skipped(entry: PathBuf) -> Self {
        Self {
            entry,
            action: SyncAction::Skipped,
        }
    }
}

impl Display for SyncOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.action {
            SyncAction::Copied { bytes } => {
                write!(f, "+ {} ({})", self.entry.display(), human_bytes(bytes))
            }
            SyncAction::Removed { bytes } => {
                write!(f, "- {} ({})", self.entry.display(), human_bytes(bytes))
            }
            SyncAction::Skipped => write!(f, "= {}", self.entry.display()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncAction {
    Copied { bytes: u64 },
    Removed { bytes: u64 },
    Skipped,
}
