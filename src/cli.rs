use clap::{Args, Parser, Subcommand, ValueEnum};
use color_eyre::Result;

use crate::commands::{issues::IssuesCommand, parcel::ParcelCommand};

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    TEXT,
    JSON,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Environment {
    Development,
    Staging,
    Sandbox,
    Production,
}

impl Environment {
    pub fn via_url(&self) -> &str {
        match self {
            Environment::Development => "http://localhost:3000",
            Environment::Staging => "https://app-staging.viaeurope.com",
            Environment::Sandbox => "https://app-sandbox.viaeurope.com",
            Environment::Production => "https://app.viaeurope.com",
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "List issues")]
    Issues(IssuesCommand),

    #[command(about = "Open parcel")]
    Parcel(ParcelCommand),
}

#[derive(Args, Debug)]
pub struct Config {
    #[arg(short, long, value_enum, default_value_t = OutputFormat::TEXT)]
    pub format: OutputFormat,

    #[arg(short, long, value_enum, alias = "env", env = "VIA_ENVIRONMENT", default_value_t = Environment::Production)]
    pub environment: Environment,

    #[arg(long, env = "VIA_LINEAR_API_KEY")]
    pub linear_api_key: String,
}

#[derive(Parser, Debug)]
#[command(bin_name = "via", name = "via")]
pub struct Via {
    #[clap(flatten)]
    config: Config,

    #[command(subcommand)]
    command: Commands,
}

impl Via {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Issues(command) => command.run(&self.config),
            Commands::Parcel(command) => command.run(&self.config),
        }
    }
}
