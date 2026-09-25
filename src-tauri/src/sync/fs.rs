use crate::sync::{compare::should_sync, types::SyncOutcome};
use anyhow::{Context, Result, anyhow};
use std::path::Path;
use tokio::fs::{copy, create_dir_all, hard_link, metadata};

/// Link or copy file
///
/// First it tries to create a hard link, if not possible then it copies the file
///
/// ## Arguments
///
/// * `src` - path to source file
/// * `dst` - path to destination file
///
/// ## Errors
///
/// Returns error if file can't be opened or read
pub async fn link_or_copy(src: &Path, dst: &Path) -> Result<()> {
    if hard_link(src, dst).await.is_err() {
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
/// ## Errors
///
/// Returns error if file can't be opened or read
///
/// ## Returns
///
/// Sync outcome
pub async fn sync_file(
    base: &Path,
    entry: &Path,
    dst: &Path,
    dry_run: bool,
) -> Result<SyncOutcome> {
    let src = base.join(entry);
    let src_meta = metadata(&src)
        .await
        .context("Failed to get metadata for source")?;

    if !src_meta.is_file() {
        return Err(anyhow!("Not a file: {}", src.display()));
    }

    let src_len = src_meta.len();
    if !should_sync(&src, dst, src_len).await? {
        return Ok(SyncOutcome::skipped(entry.to_path_buf()));
    }

    if !dry_run {
        if let Some(parent) = dst.parent() {
            create_dir_all(parent).await.context(anyhow!(
                "Failed to create parent directory for: {}",
                dst.display()
            ))?;
        }

        link_or_copy(&src, dst).await.context(anyhow!(
            "Failed to link or copy `{}` to `{}`",
            src.display(),
            dst.display()
        ))?;
    }

    Ok(SyncOutcome::copied(entry.to_path_buf(), src_meta.len()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::types::SyncAction;
    use tempfile::tempdir;
    use tokio::fs::{read, read_to_string, write};

    #[tokio::test]
    async fn test_link_or_copy() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let temp_path = temp_dir.path();
        let src = temp_path.join("src");
        let dst = temp_path.join("dst");

        write(&src, "test").await.expect("Failed to write to src");
        link_or_copy(&src, &dst)
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
        let outcome = sync_file(src_dir.path(), &relative_entry, &full_dst, false)
            .await
            .unwrap();

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
        let outcome = sync_file(src_dir.path(), relative_entry, &full_dst, true)
            .await
            .unwrap();

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

        let outcome = sync_file(src_dir.path(), relative_entry, &full_dst, false)
            .await
            .unwrap();

        assert_eq!(outcome.action, SyncAction::Skipped);
    }
}
