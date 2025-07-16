// src/main.rs
/*
 * Main executable for AetherForecast
 */

use clap::Parser;
use aetherforecast::{Result, run};

#[derive(Parser)]
#[command(version, about = "AetherForecast - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
