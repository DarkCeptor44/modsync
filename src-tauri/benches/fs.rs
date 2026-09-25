use anyhow::Result;
use divan::{Bencher, black_box, counter::ItemsCount};
use modsync_lib::sync::fs as lib;
use std::{
    fs::{create_dir_all, write},
    path::PathBuf,
};
use tempfile::{TempDir, tempdir};
use tokio::runtime::Runtime;

const ARGS: &[usize] = &[50, 500, 1000];
const MAX_DEPTH: usize = 10;

fn main() {
    divan::main();
}

#[divan::bench(args = ARGS)]
fn collect_files(b: Bencher, n: usize) {
    let rt = Runtime::new().unwrap();
    let dir = setup_test_dirs(n, MAX_DEPTH).unwrap();
    let handle = rt.handle();

    b.counter(ItemsCount::new(n)).bench(|| {
        handle.block_on(async { black_box(lib::collect_source_files(dir.path(), &[]).await) })
    });
}

fn setup_test_dirs(file_count: usize, max_depth: usize) -> Result<TempDir> {
    let temp_dir = tempdir()?;
    let root = temp_dir.path();

    for i in 0..file_count {
        let depth = i % max_depth;
        let mut dir_path = PathBuf::from(root);

        for d in 0..depth {
            dir_path.push(format!("folder_{}", d));
        }

        create_dir_all(&dir_path)?;

        let file_path = dir_path.join(format!("file_{}.pak", i));
        write(file_path, b"dummy content for benchmark")?;
    }

    Ok(temp_dir)
}
