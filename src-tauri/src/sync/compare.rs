use anyhow::Result;
use std::{hash::Hasher, path::Path};
use tokio::{
    fs::{File, metadata},
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
};
use twox_hash::XxHash64;

const FULL_CHECK_THRESHOLD: u64 = 50 * 1024 * 1024; // 50 MB
const QUICK_CHECK_SIZE: usize = 4096;

/// Calculate full hash for file
///
/// ## Arguments
///
/// * `path` - path to file
///
/// ## Errors
///
/// Returns error if file can't be opened or read
///
/// ## Returns
///
/// Hash of file
pub async fn calculate_full_hash(path: &Path) -> Result<u64> {
    let mut file = File::open(path).await?;
    let mut hasher = XxHash64::default();
    let mut buffer = vec![0u8; 65536].into_boxed_slice();

    loop {
        let bytes_read = file.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }
        hasher.write(&buffer[..bytes_read]);
    }

    Ok(hasher.finish())
}

/// Calculate quick hash for file
///
/// ## Arguments
///
/// * `path` - path to file
///
/// ## Errors
///
/// Returns error if file can't be opened or read
///
/// ## Returns
///
/// Hash of file
pub async fn calculate_quick_hash(path: &Path) -> Result<u64> {
    let file = File::open(path).await?;
    let mut buffer = BufReader::with_capacity(QUICK_CHECK_SIZE, file);
    let bytes_read = buffer.fill_buf().await?;
    Ok(XxHash64::oneshot(0, bytes_read))
}

/// Check if file should be synced
///
/// ## Arguments
///
/// * `src` - path to source file
/// * `dst` - path to destination file
/// * `src_len` - length of source file
///
/// ## Errors
///
/// Returns error if file can't be opened or read
///
/// ## Returns
///
/// True if file should be synced
pub async fn should_sync(src: &Path, dst: &Path, src_len: u64) -> Result<bool> {
    let Ok(dst_meta) = metadata(dst).await else {
        return Ok(true);
    };

    if src_len != dst_meta.len() {
        return Ok(true);
    }

    let src_mtime = metadata(src).await?.modified().ok();
    let dst_mtime = dst_meta.modified().ok();
    if src_mtime.is_some() && src_mtime == dst_mtime {
        return Ok(false);
    }

    let src_quick = calculate_quick_hash(src).await?;
    let dst_quick = calculate_quick_hash(dst).await?;

    if src_quick != dst_quick {
        return Ok(true);
    }

    // if file is <= 50MB, do a full content hash as a final check
    if src_len <= FULL_CHECK_THRESHOLD {
        let src_full = calculate_full_hash(src).await?;
        let dst_full = calculate_full_hash(dst).await?;
        return Ok(src_full != dst_full);
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Write, time::Duration};
    use tempfile::NamedTempFile;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_full_hash_consistency() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"consistent data payload").unwrap();
        file.flush().unwrap();

        let hash1 = calculate_full_hash(file.path()).await.unwrap();
        let hash2 = calculate_full_hash(file.path()).await.unwrap();

        assert_eq!(
            hash1, hash2,
            "Hashing the same unchanged file twice must yield identical results"
        );
    }

    #[tokio::test]
    async fn test_full_hash_detects_tail_changes() {
        let mut file1 = NamedTempFile::new().unwrap();
        let mut file2 = NamedTempFile::new().unwrap();

        let prefix = vec![0xAA; 4096];

        file1.write_all(&prefix).unwrap();
        file1.write_all(b"tail data AAAAAA").unwrap();
        file1.flush().unwrap();

        file2.write_all(&prefix).unwrap();
        file2.write_all(b"tail data BBBBBB").unwrap();
        file2.flush().unwrap();

        let hash1 = calculate_full_hash(file1.path()).await.unwrap();
        let hash2 = calculate_full_hash(file2.path()).await.unwrap();

        assert_ne!(
            hash1, hash2,
            "Full hash must differ when content after 4KB changes"
        );
    }

    #[tokio::test]
    async fn test_should_sync_different_mtime_identical_content() {
        let mut src = NamedTempFile::new().unwrap();
        let mut dst = NamedTempFile::new().unwrap();

        let payload = b"identical file content across two temp files";
        src.write_all(payload).unwrap();
        src.flush().unwrap();

        dst.write_all(payload).unwrap();
        dst.flush().unwrap();

        let result = should_sync(src.path(), dst.path(), payload.len() as u64)
            .await
            .unwrap();

        assert!(
            !result,
            "Should skip when content matches, even with different mtimes"
        );
    }

    #[tokio::test]
    async fn test_should_sync_different_tail_under_threshold() {
        let prefix = vec![0xAA; QUICK_CHECK_SIZE];

        let mut src = NamedTempFile::new().unwrap();
        src.write_all(&prefix).unwrap();
        src.write_all(b"SOURCE_TAIL_DATA").unwrap();
        src.flush().unwrap();

        sleep(Duration::from_millis(10)).await;

        let mut dst = NamedTempFile::new().unwrap();
        dst.write_all(&prefix).unwrap();
        dst.write_all(b"TARGET_TAIL_DATA").unwrap();
        dst.flush().unwrap();

        let src_len = (QUICK_CHECK_SIZE + 16) as u64;
        let result = should_sync(src.path(), dst.path(), src_len).await.unwrap();

        assert!(
            result,
            "Should trigger sync when full hash catches tail changes under 50MB"
        );
    }

    #[tokio::test]
    async fn test_should_sync_missing_destination() {
        let mut src = NamedTempFile::new().unwrap();
        src.write_all(b"some payload").unwrap();
        src.flush().unwrap();

        let dst_path = src
            .path()
            .parent()
            .unwrap()
            .join("non_existent_dst_file.tmp");
        let result = should_sync(src.path(), &dst_path, 12).await.unwrap();
        assert!(result, "Should sync when destination file does not exist");
    }

    #[tokio::test]
    async fn test_should_sync_mtime_match() {
        let mut src = NamedTempFile::new().unwrap();
        src.write_all(b"identical content").unwrap();
        src.flush().unwrap();

        let result = should_sync(src.path(), src.path(), 17).await.unwrap();
        assert!(!result, "Should skip when size and mtime are identical");
    }

    #[tokio::test]
    async fn test_quick_hash_reads_only_prefix() {
        let mut file1 = NamedTempFile::new().unwrap();
        let mut file2 = NamedTempFile::new().unwrap();

        let prefix = vec![0xAA; 4096];

        file1.write_all(&prefix).unwrap();
        file1.write_all(b"tail data AAAAAA").unwrap();
        file1.flush().unwrap();

        file2.write_all(&prefix).unwrap();
        file2.write_all(b"tail data BBBBBB").unwrap();
        file2.flush().unwrap();

        let hash1 = calculate_quick_hash(file1.path()).await.unwrap();
        let hash2 = calculate_quick_hash(file2.path()).await.unwrap();

        assert_eq!(
            hash1, hash2,
            "Quick hash should match when first 4KB are identical"
        );
    }
}
