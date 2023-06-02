#![allow(unused)]

mod modules;

use anyhow::Context;
use clap::Parser;
use log::info;

use crate::modules::read_file::read;
use crate::modules::write_buffer::write;

/// Structure to hold data.
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    // Set an env var to show verbose log.
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "info");

    // Log our init message
    env_logger::init();
    info!("starting program");

    let args = Cli::parse();
    let content = read(&args.path);

    match content {
        Ok(val) => {
            grrs::find_matches(&val, &args.pattern, &mut std::io::stdout());
        }
        Err(e) => {
            eprintln!("Error! {:?}", e);
        }
    }

    Ok(())
}
