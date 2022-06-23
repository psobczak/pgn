mod cli;
mod pgn;

use clap::Parser;
use cli::Args;
use pgn::Pgn;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("{:?}", args);
    let pgn = Pgn::new("lichess_pgn_2022.05.14_MontagueT_vs_sobka_sobka.w1W5Uk5m.pgn")?;

    for line in pgn.tags() {
        println!("{:?}", line);
    }

    Ok(())
}
