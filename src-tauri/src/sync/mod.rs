pub mod compare;
pub mod fs;
pub mod types;

use crate::{
    sync::{
        fs::{collect_empty_dirs, collect_files, handle_dir_removal, handle_removal, sync_file},
        types::{SyncAction, SyncOutcome},
    },
    types::Profile,
};
use anyhow::{Context, Result, anyhow};
use std::{collections::HashSet, path::PathBuf, sync::Arc};
use tokio::{
    fs::create_dir_all,
    sync::{Semaphore, mpsc::UnboundedSender},
    task::JoinSet,
};

/// Sync profile
///
/// ## Arguments
///
/// * `profile` - profile to sync
/// * `tx` - channel to send sync outcomes
/// * `concurrency_limit` - concurrency limit
/// * `dry_run` - dry run, does not write files
///
/// ## Errors
///
/// Returns error if sync fails
pub async fn sync(
    profile: &Profile,
    tx: UnboundedSender<SyncOutcome>,
    concurrency_limit: usize,
    dry_run: bool,
) -> Result<()> {
    let src = Arc::new(profile.source.clone());
    let dst = Arc::new(profile.destination.clone());

    if !src.is_dir() {
        return Err(anyhow!("Source folder does not exist: {}", src.display()));
    }

    create_dir_all(&*dst)
        .await
        .context("Failed to create destination folder")?;

    let src_files: Arc<HashSet<PathBuf>> =
        Arc::new(collect_files(&src, &profile.sync_exclusions).await?);

    // 1st pass: sync files from src to dst
    let semaphore = Arc::new(Semaphore::new(concurrency_limit));
    let mut set: JoinSet<Result<SyncOutcome>> = JoinSet::new();
    let src_files_ref = src_files.clone();

    for entry in src_files_ref.iter() {
        let semaphore_clone = semaphore.clone();
        let src_clone = src.clone();
        let dst_clone = dst.clone();
        let entry_clone = entry.clone();

        set.spawn(async move {
            let _permit = semaphore_clone.acquire_owned().await?;
            let dst_path = dst_clone.join(&entry_clone);

            Ok(sync_file(&src_clone, &entry_clone, &dst_path, dry_run).await)
        });
    }

    while let Some(res) = set.join_next().await {
        match res {
            Ok(Ok(outcome)) => {
                let _ = tx.send(outcome);
            }
            Ok(Err(err)) => {
                let _ = tx.send(SyncOutcome {
                    entry: PathBuf::from("unknown"),
                    action: SyncAction::Failed {
                        error: err.to_string(),
                    },
                });
            }
            Err(join_err) => {
                let _ = tx.send(SyncOutcome {
                    entry: PathBuf::from("unknown"),
                    action: SyncAction::Failed {
                        error: format!("Task panicked: {join_err}"),
                    },
                });
            }
        }
    }

    let dst_files = collect_files(&dst, &profile.delete_exclusions).await?;
    let files_to_remove: HashSet<PathBuf> = dst_files.difference(&src_files).cloned().collect();

    // 2nd pass: remove dst files that are not in src
    let semaphore = Arc::new(Semaphore::new(concurrency_limit));
    let mut set: JoinSet<Result<SyncOutcome>> = JoinSet::new();

    for entry in &files_to_remove {
        let semaphore = semaphore.clone();
        let entry_clone = entry.clone();
        let dst_clone = dst.clone();

        set.spawn(async move {
            let _permit = semaphore.acquire_owned().await?;

            Ok(handle_removal(&dst_clone, &entry_clone, dry_run).await)
        });
    }

    while let Some(res) = set.join_next().await {
        match res {
            Ok(Ok(outcome)) => {
                let _ = tx.send(outcome);
            }
            Ok(Err(err)) => {
                let _ = tx.send(SyncOutcome {
                    entry: PathBuf::from("unknown"),
                    action: SyncAction::Failed {
                        error: err.to_string(),
                    },
                });
            }
            Err(join_err) => {
                let _ = tx.send(SyncOutcome {
                    entry: PathBuf::from("unknown"),
                    action: SyncAction::Failed {
                        error: format!("Task panicked: {join_err}"),
                    },
                });
            }
        }
    }

    // 3rd pass: remove empty directories left over
    let semaphore = Arc::new(Semaphore::new(concurrency_limit));
    let mut set: JoinSet<Result<SyncOutcome>> = JoinSet::new();

    let dirs_to_remove = collect_empty_dirs(&dst, &profile.delete_exclusions).await?;

    for entry in &dirs_to_remove {
        let semaphore = semaphore.clone();
        let entry_clone = entry.clone();
        let dst_clone = dst.clone();

        set.spawn(async move {
            let _permit = semaphore.acquire_owned().await?;

            Ok(handle_dir_removal(&dst_clone, &entry_clone, dry_run).await)
        });
    }

    while let Some(res) = set.join_next().await {
        match res {
            Ok(Ok(outcome)) => {
                let _ = tx.send(outcome);
            }
            Ok(Err(err)) => {
                let _ = tx.send(SyncOutcome {
                    entry: PathBuf::from("unknown"),
                    action: SyncAction::Failed {
                        error: err.to_string(),
                    },
                });
            }
            Err(join_err) => {
                let _ = tx.send(SyncOutcome {
                    entry: PathBuf::from("unknown"),
                    action: SyncAction::Failed {
                        error: format!("Task panicked: {join_err}"),
                    },
                });
            }
        }
    }

    Ok(())
}
