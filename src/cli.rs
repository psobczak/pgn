use clap::Parser;

/// Simple program that tries to read and parse .pgn files
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}
