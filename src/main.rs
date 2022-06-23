mod cli;
mod pgn;

use clap::Parser;
use cli::Args;
use pgn::Pgn;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let pgn = Pgn::new(args.path)?;

    for line in pgn.tags() {
        println!("{:?}", line);
    }

    Ok(())
}
