// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use divan::{
    Bencher, black_box,
    counter::{BytesCount, ItemsCount},
};
use modsync_lib::sync::compare::{self as lib, QUICK_CHECK_SIZE};
use std::io::Cursor;
use tokio::runtime::Runtime;

const ARGS: &[usize] = &[65_536, 1_048_576, 10_485_760];

fn main() {
    divan::main();
}

#[divan::bench(args = ARGS)]
fn calculate_full_hash(b: Bencher, size: usize) {
    let rt = Runtime::new().unwrap();
    let handle = rt.handle();
    let payload = vec![0xAB; size];

    b.counter(BytesCount::new(size))
        .counter(ItemsCount::new(1usize))
        .bench(|| {
            let cursor = Cursor::new(&payload);

            handle.block_on(async { black_box(lib::calculate_full_hash(cursor).await).unwrap() })
        });
}

#[divan::bench(args = ARGS)]
fn calculate_quick_hash(b: Bencher, size: usize) {
    let rt = Runtime::new().unwrap();
    let handle = rt.handle();
    let payload = vec![0xAB; size];

    b.counter(BytesCount::new(QUICK_CHECK_SIZE))
        .counter(ItemsCount::new(1usize))
        .bench(|| {
            let cursor = Cursor::new(&payload);

            handle.block_on(async { black_box(lib::calculate_quick_hash(cursor).await).unwrap() })
        });
}
