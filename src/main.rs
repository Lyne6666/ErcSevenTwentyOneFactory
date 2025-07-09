// src/main.rs
/*
 * Main executable for ErcSevenTwentyOneFactory
 */

use clap::Parser;
use ercseventwentyonefactory::{Result, run};

#[derive(Parser)]
#[command(version, about = "ErcSevenTwentyOneFactory - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
