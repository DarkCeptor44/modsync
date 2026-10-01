// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod compare;
pub mod fs;
pub mod types;

use crate::{
    sync::{
        fs::{collect_empty_dirs, collect_files, handle_dir_removal, handle_removal, sync_file},
        types::SyncOutcome,
    },
    types::{Profile, Settings},
};
use anyhow::{Context, Result, anyhow};
use futures::{StreamExt, stream::iter};
use std::{collections::HashSet, path::PathBuf, pin::Pin, sync::Arc};
use tokio::{fs::create_dir_all, sync::mpsc::UnboundedSender};

async fn run_parallel_pass<F>(
    items: &HashSet<PathBuf>,
    jobs: usize,
    tx: &UnboundedSender<SyncOutcome>,
    work: F,
) where
    F: Fn(PathBuf) -> Pin<Box<dyn Future<Output = SyncOutcome> + Send>> + Send + Sync,
{
    let mut stream = iter(items.iter().cloned()).map(work).buffer_unordered(jobs);

    while let Some(outcome) = stream.next().await {
        let _ = tx.send(outcome);
    }
}

/// Sync profile
///
/// ## Arguments
///
/// * `profile` - profile to sync
/// * `tx` - channel to send sync outcomes
/// * `settings` - global settings
/// * `dry_run` - dry run, does not write files
///
/// ## Errors
///
/// Returns error if sync fails
pub async fn sync(
    profile: &Profile,
    tx: UnboundedSender<SyncOutcome>,
    settings: Settings,
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

    let src_files = collect_files(&src, &profile.sync_exclusions).await?;

    // 1st pass: sync files from src to dst
    run_parallel_pass(&src_files, settings.jobs, &tx, |entry| {
        let src = src.clone();
        let dst = dst.clone();

        Box::pin(async move {
            let dst_path = dst.join(&entry);

            sync_file(&src, &entry, &dst_path, dry_run).await
        })
    })
    .await;

    // 2nd pass: remove dst files that are not in src
    let dst_files = collect_files(&dst, &profile.delete_exclusions).await?;
    let files_to_remove: HashSet<PathBuf> = dst_files.difference(&src_files).cloned().collect();

    run_parallel_pass(&files_to_remove, settings.jobs, &tx, |entry| {
        let dst = dst.clone();
        Box::pin(async move { handle_removal(&dst, &entry, dry_run).await })
    })
    .await;

    // 3rd pass: remove empty directories left over
    let dirs_to_remove = collect_empty_dirs(&dst, &profile.delete_exclusions).await?;

    for entry in dirs_to_remove {
        let outcome = handle_dir_removal(&dst, &entry, dry_run).await;
        let _ = tx.send(outcome);
    }

    Ok(())
}
