// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::sync::{
    compare::should_sync,
    types::{SyncAction, SyncOutcome},
};
use anyhow::{Context, Result, anyhow};
use std::{
    collections::HashSet,
    fs::read_dir,
    path::{Path, PathBuf},
};
use tokio::{
    fs::{copy, create_dir_all, hard_link, metadata, remove_dir, remove_file, symlink_metadata},
    task::spawn_blocking,
};
use walkdir::WalkDir;

/// Collect empty directories
///
/// ## Arguments
///
/// * `root` - path to root directory
/// * `exclusions` - list of paths to exclude
///
/// ## Errors
///
/// Returns error if task join fails
///
/// ## Returns
///
/// Set of empty directories
pub async fn collect_empty_dirs(root: &Path, exclusions: &[PathBuf]) -> Result<HashSet<PathBuf>> {
    let root = root.to_path_buf();
    let exclusions = exclusions.to_vec();

    spawn_blocking(move || {
        let mut set = HashSet::new();

        for entry in WalkDir::new(&root)
            .min_depth(1)
            .contents_first(true)
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            let path = entry.path();
            let Ok(rel) = path.strip_prefix(&root) else {
                continue;
            };

            if exclusions.iter().any(|ex| rel.starts_with(ex) || rel == ex) {
                continue;
            }

            if entry.file_type().is_dir() && is_dir_empty(path) {
                set.insert(rel.to_path_buf());
            }
        }

        Ok(set)
    })
    .await
    .context("Task join failed")?
}

/// Collect files
///
/// ## Arguments
///
/// * `root` - path to root directory
/// * `exclusions` - list of paths to exclude
///
/// ## Errors
///
/// Returns error if task join failed
///
/// ## Returns
///
/// Set of files
pub async fn collect_files(root: &Path, exclusions: &[PathBuf]) -> Result<HashSet<PathBuf>> {
    let root = root.to_path_buf();
    let exclusions = exclusions.to_vec();

    spawn_blocking(move || {
        let mut set = HashSet::new();

        for entry in WalkDir::new(&root)
            .min_depth(1)
            .into_iter()
            .filter_entry(|e| {
                if let Ok(rel) = e.path().strip_prefix(&root) {
                    !exclusions.iter().any(|ex| rel.starts_with(ex))
                } else {
                    true
                }
            })
        {
            let entry = entry.context("Failed to read entry")?;
            if entry.file_type().is_file()
                && let Ok(rel_path) = entry.path().strip_prefix(&root)
            {
                set.insert(rel_path.to_path_buf());
            }
        }

        Ok(set)
    })
    .await
    .context("Task join failed")?
}

/// Removes file or link if it exists
///
/// ## Arguments
///
/// * `base` - base path
/// * `entry` - entry to remove
/// * `dry_run` - dry run
///
/// ## Returns
///
/// The outcome of the removal operation
pub async fn handle_removal(base: &Path, entry: &Path, dry_run: bool) -> SyncOutcome {
    async fn inner(base: &Path, entry: &Path, dry_run: bool) -> Result<SyncAction> {
        let dst = base.join(entry);
        let dst_meta = symlink_metadata(&dst)
            .await
            .context(anyhow!("Failed to read metadata for: {}", dst.display()))?;

        if !dst_meta.is_file() && !dst_meta.is_symlink() {
            return Err(anyhow!("Not a file or symlink: {}", dst.display()));
        }

        if !dry_run {
            remove_file(&dst)
                .await
                .context(anyhow!("Failed to remove file: {}", dst.display()))?;
        }

        Ok(SyncAction::Removed {
            bytes: dst_meta.len(),
        })
    }

    match inner(base, entry, dry_run).await {
        Ok(action) => SyncOutcome {
            entry: entry.to_path_buf(),
            action,
        },
        Err(e) => SyncOutcome {
            entry: entry.to_path_buf(),
            action: SyncAction::Failed {
                error: e.to_string(),
            },
        },
    }
}

pub async fn handle_dir_removal(base: &Path, entry: &Path, dry_run: bool) -> SyncOutcome {
    async fn inner(base: &Path, entry: &Path, dry_run: bool) -> Result<SyncAction> {
        let dst = base.join(entry);
        let dst_meta = metadata(&dst)
            .await
            .context(anyhow!("Failed to read metadata for: {}", dst.display()))?;

        if !dst_meta.is_dir() {
            return Err(anyhow!("Not a directory: {}", dst.display()));
        }

        if !is_dir_empty(&dst) {
            return Err(anyhow!("Directory is not empty: {}", dst.display()));
        }

        if !dry_run {
            remove_dir(&dst)
                .await
                .context(anyhow!("Failed to remove directory: {}", dst.display()))?;
        }

        Ok(SyncAction::RemovedDir)
    }

    match inner(base, entry, dry_run).await {
        Ok(action) => SyncOutcome {
            entry: entry.to_path_buf(),
            action,
        },
        Err(e) => SyncOutcome {
            entry: entry.to_path_buf(),
            action: SyncAction::Failed {
                error: e.to_string(),
            },
        },
    }
}

fn is_dir_empty(path: &Path) -> bool {
    read_dir(path).is_ok_and(|mut entries| entries.next().is_none())
}

/// Link or copy file
///
/// First it tries to create a hard link, if not possible then it copies the file. You can also force a copy with `force_copy`
///
/// ## Arguments
///
/// * `src` - path to source file
/// * `dst` - path to destination file
/// * `force_copy` - force copy
///
/// ## Errors
///
/// Returns error if file can't be linked or copied
pub async fn link_or_copy(src: &Path, dst: &Path, force_copy: bool) -> Result<()> {
    let link_failed = if force_copy {
        true
    } else {
        hard_link(src, dst).await.is_err()
    };

    if link_failed {
        copy(src, dst).await.context(anyhow!(
            "Failed to copy: {} -> {}",
            src.display(),
            dst.display()
        ))?;
    }
    Ok(())
}

/// Sync file
///
/// ## Arguments
///
/// * `base` - base path
/// * `entry` - entry to sync
/// * `dst` - destination path
/// * `dry_run` - dry run
///
/// ## Returns
///
/// Outcome for the sync operation
pub async fn sync_file(base: &Path, entry: &Path, dst: &Path, dry_run: bool) -> SyncOutcome {
    async fn inner(base: &Path, entry: &Path, dst: &Path, dry_run: bool) -> Result<SyncAction> {
        let src = base.join(entry);
        let src_meta = metadata(&src)
            .await
            .context("Failed to get metadata for source")?;

        if !src_meta.is_file() {
            return Err(anyhow!("Not a file: {}", src.display()));
        }

        let src_len = src_meta.len();
        if !should_sync(&src, dst, src_len).await? {
            return Ok(SyncAction::Skipped);
        }

        if !dry_run {
            if let Some(parent) = dst.parent() {
                create_dir_all(parent).await.context(anyhow!(
                    "Failed to create parent directory for: {}",
                    dst.display()
                ))?;
            }

            link_or_copy(&src, dst, false).await.context(anyhow!(
                "Failed to link or copy `{}` to `{}`",
                src.display(),
                dst.display()
            ))?;
        }

        Ok(SyncAction::Copied {
            bytes: src_meta.len(),
        })
    }

    match inner(base, entry, dst, dry_run).await {
        Ok(action) => SyncOutcome {
            entry: entry.to_path_buf(),
            action,
        },
        Err(e) => SyncOutcome {
            entry: entry.to_path_buf(),
            action: SyncAction::Failed {
                error: e.to_string(),
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::types::SyncAction;
    use tempfile::tempdir;
    use tokio::fs::{create_dir, read, read_to_string, write};

    #[tokio::test]
    async fn test_collect_empty_dirs_conflict() {
        let temp_dir = tempdir().unwrap();
        let root = temp_dir.path();

        create_dir_all(root.join("subfolder")).await.unwrap();
        write(root.join("subfolder").join("file1.txt"), "data")
            .await
            .unwrap();

        let dirs = collect_empty_dirs(root, &[]).await.unwrap();
        assert_eq!(dbg!(&dirs).len(), 0);
        assert!(!dirs.contains(&PathBuf::from("subfolder")));
    }

    #[tokio::test]
    async fn test_collect_empty_dirs_success() {
        let temp_dir = tempdir().unwrap();
        let root = temp_dir.path();

        create_dir_all(root.join("subfolder")).await.unwrap();
        create_dir_all(root.join("excluded_folder")).await.unwrap();

        let exclusions = vec![PathBuf::from("excluded_folder")];
        let dirs = collect_empty_dirs(root, &exclusions).await.unwrap();

        assert_eq!(dbg!(&dirs).len(), 1);
        assert!(dirs.contains(&PathBuf::from("subfolder")));
        assert!(!dirs.contains(&PathBuf::from("excluded_folder")));
    }

    #[tokio::test]
    async fn test_collect_files() {
        let temp_dir = tempdir().unwrap();
        let root = temp_dir.path();

        create_dir_all(root.join("subfolder")).await.unwrap();
        create_dir_all(root.join("excluded_folder")).await.unwrap();

        write(root.join("file1.txt"), "data").await.unwrap();
        write(root.join("subfolder").join("file2.txt"), "data")
            .await
            .unwrap();
        write(root.join("excluded_folder").join("file3.txt"), "data")
            .await
            .unwrap();
        write(root.join("skip_me.log"), "data").await.unwrap();

        let exclusions = vec![
            PathBuf::from("excluded_folder"),
            PathBuf::from("skip_me.log"),
        ];

        let files = collect_files(root, &exclusions).await.unwrap();

        assert_eq!(files.len(), 2);
        assert!(files.contains(&PathBuf::from("file1.txt")));
        assert!(files.contains(&PathBuf::from("subfolder").join("file2.txt")));

        assert!(!files.contains(&PathBuf::from("excluded_folder").join("file3.txt")));
        assert!(!files.contains(&PathBuf::from("skip_me.log")));
    }

    #[tokio::test]
    async fn test_handle_dir_removal_conflict() {
        let temp_dir = tempdir().unwrap();
        let base = temp_dir.path();
        let relative_entry = Path::new("dir_to_remove");

        let full_path = base.join(relative_entry);
        create_dir(&full_path).await.unwrap();
        write(full_path.join("file1.txt"), b"some data")
            .await
            .unwrap();
        assert!(full_path.is_dir());

        let outcome = handle_dir_removal(base, relative_entry, false).await;
        assert!(
            matches!(outcome.action, SyncAction::Failed { .. }),
            "Expected SyncAction::Failed for non-empty directory, got {:?}",
            outcome.action
        );
    }

    #[tokio::test]
    async fn test_handle_dir_removal_empty() {
        let temp_dir = tempdir().unwrap();
        let base = temp_dir.path();
        let relative_entry = Path::new("dir_to_remove");

        let full_path = base.join(relative_entry);
        create_dir(&full_path).await.unwrap();

        let outcome = handle_dir_removal(base, relative_entry, false).await;
        assert_eq!(
            outcome.action,
            SyncAction::RemovedDir,
            "Expected SyncAction::RemovedDir for empty directory, got {:?}",
            outcome.action
        );
        assert!(!full_path.is_dir());
    }

    #[tokio::test]
    async fn test_handle_removal_file() {
        let temp_dir = tempdir().unwrap();
        let base = temp_dir.path();
        let relative_entry = Path::new("file_to_remove.txt");
        let target_path = base.join(relative_entry);

        write(&target_path, "test contents").await.unwrap();
        assert!(target_path.is_file());

        let outcome = handle_removal(base, relative_entry, false).await;
        match outcome.action {
            SyncAction::Removed { bytes } => {
                assert_eq!(bytes, 13);
            }
            _ => panic!("Expected SyncAction::Removed, got {:?}", outcome.action),
        }

        assert!(!target_path.is_file());
    }

    #[tokio::test]
    async fn test_handle_removal_symlink() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let base = temp_dir.path();
        let relative_entry = Path::new("symlink_to_remove");
        let symlink_path = base.join(relative_entry);

        #[cfg(unix)]
        tokio::fs::symlink("non_existent_target.txt", &symlink_path)
            .await
            .unwrap();

        #[cfg(windows)]
        tokio::fs::symlink_file("non_existent_target.txt", &symlink_path)
            .await
            .unwrap();

        assert!(symlink_metadata(&symlink_path).await.is_ok());

        let outcome = handle_removal(base, relative_entry, false).await;
        match outcome.action {
            SyncAction::Removed { .. } => {}
            _ => panic!("Expected SyncAction::Removed, got {:?}", outcome.action),
        }

        assert!(symlink_metadata(&symlink_path).await.is_err());
    }

    #[tokio::test]
    async fn test_link_or_copy_copy() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let temp_path = temp_dir.path();
        let src = temp_path.join("src");
        let dst = temp_path.join("dst");

        write(&src, "test").await.expect("Failed to write to src");
        link_or_copy(&src, &dst, true)
            .await
            .expect("Failed to copy");

        assert_eq!(
            read_to_string(&dst).await.expect("Failed to read dst"),
            "test"
        );
    }

    #[tokio::test]
    async fn test_link_or_copy_link() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let temp_path = temp_dir.path();
        let src = temp_path.join("src");
        let dst = temp_path.join("dst");

        write(&src, "test").await.expect("Failed to write to src");
        link_or_copy(&src, &dst, false)
            .await
            .expect("Failed to link or copy");

        assert_eq!(
            read_to_string(&dst).await.expect("Failed to read dst"),
            "test"
        );
    }

    #[tokio::test]
    async fn test_sync_file_copies_new_file() {
        let src_dir = tempdir().unwrap();
        let dst_dir = tempdir().unwrap();

        let relative_entry = Path::new("nested").join("test_mod.pak");
        let full_src = src_dir.path().join(&relative_entry);

        create_dir_all(full_src.parent().unwrap()).await.unwrap();
        write(&full_src, b"mod data payload").await.unwrap();

        let full_dst = dst_dir.path().join(&relative_entry);
        let outcome = sync_file(src_dir.path(), &relative_entry, &full_dst, false).await;

        assert_eq!(outcome.entry, relative_entry);
        assert_eq!(outcome.action, SyncAction::Copied { bytes: 16 });
        assert!(full_dst.is_file(), "Destination file must be created");
        assert_eq!(read(&full_dst).await.unwrap(), b"mod data payload");
    }

    #[tokio::test]
    async fn test_sync_file_dry_run_does_not_write() {
        let src_dir = tempdir().unwrap();
        let dst_dir = tempdir().unwrap();

        let relative_entry = Path::new("dry_run_mod.pak");
        let full_src = src_dir.path().join(relative_entry);
        write(&full_src, b"dry run payload").await.unwrap();

        let full_dst = dst_dir.path().join(relative_entry);
        let outcome = sync_file(src_dir.path(), relative_entry, &full_dst, true).await;

        assert_eq!(outcome.action, SyncAction::Copied { bytes: 15 });
        assert!(
            !full_dst.is_file(),
            "File should NOT be written in dry run mode"
        );
    }

    #[tokio::test]
    async fn test_sync_file_skips_unchanged() {
        let src_dir = tempdir().unwrap();
        let dst_dir = tempdir().unwrap();

        let relative_entry = Path::new("mod.pak");
        let full_src = src_dir.path().join(relative_entry);
        let full_dst = dst_dir.path().join(relative_entry);

        let payload = b"identical mod content";
        write(&full_src, payload).await.unwrap();
        write(&full_dst, payload).await.unwrap();

        let outcome = sync_file(src_dir.path(), relative_entry, &full_dst, false).await;

        assert_eq!(outcome.action, SyncAction::Skipped);
    }
}
