use std::path::PathBuf;

use clap::Parser;

/// Simple program that tries to read and parse .pgn files
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Path to the .png file
    #[clap(short, long, value_parser)]
    pub path: PathBuf,
}
