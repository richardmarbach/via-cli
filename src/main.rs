use clap::Parser;
use color_eyre::Result;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use via::commands::Cli;

fn main() -> Result<()> {
    color_eyre::install()?;

    let config: Cli = Figment::new()
        .merge(Toml::file("config.toml"))
        .merge(Env::prefixed("VIA_CLI_").split("__"))
        .merge(Serialized::defaults(Cli::parse()))
        .extract()?;

    config.run();

    Ok(())
}
