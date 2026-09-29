// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use colored::Colorize;
use dotenvy::dotenv;
use std::process::exit;

const NAME: &str = env!("CARGO_BIN_NAME");

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct App {
    #[arg(
        long,
        help = "Print debugging information",
        env = "DEBUG",
        default_value_t
    )]
    debug: bool,
}

fn main() {
    dotenv().ok();

    let args = App::parse();
    if let Err(e) = modsync_lib::run(args.debug || cfg!(debug_assertions)) {
        eprintln!("{}", format!("{NAME}: {e:?}").red());
        exit(1);
    }
}
