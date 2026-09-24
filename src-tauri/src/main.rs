// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use dotenvy::dotenv;

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
    modsync_lib::run(args.debug || cfg!(debug_assertions))
}
