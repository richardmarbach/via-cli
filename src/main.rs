use clap::Parser;
use color_eyre::Result;
use via::cli::Via;

fn main() -> Result<()> {
    color_eyre::install()?;

    let via = Via::parse();
    via.run()
}
