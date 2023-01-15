use crate::cli::Config;
use clap::{Args, Subcommand};
use color_eyre::{eyre::Context, Result};

#[derive(Args, Debug)]
pub struct ParcelCommand {
    #[command(subcommand)]
    command: Commands,
}

impl ParcelCommand {
    pub fn run(&self, config: &Config) -> Result<()> {
        match &self.command {
            Commands::Open(command) => command.run(&config),
        }
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Open the parcel in the browser")]
    Open(OpenCommand),
}

#[derive(Args, Debug)]
pub struct OpenCommand {
    #[arg(help = "Parcel ref")]
    reference: String,
}

impl OpenCommand {
    pub fn run(&self, config: &Config) -> Result<()> {
        let url = format!(
            "{}/parcels/{}",
            config.environment.via_url(),
            self.reference
        );
        open::that(url)
            .with_context(|| format!("Failed to open parcel in browser {}", self.reference))
    }
}
