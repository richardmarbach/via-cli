use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

use crate::via;
use crate::linear::LinearClient;

#[derive(Parser, Serialize, Deserialize, Debug)]
/// The handy ViaEurope CLI
pub struct Cli {
    #[clap(flatten)]
    linear: LinearConfig,

    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn run(&self) {
        match &self.command {
            Commands::Issues(_) => todo!(),
            Commands::Issue(issue) => match &issue.command {
                IssueCommands::View(command) => command.run(&self),
            },
            Commands::Parcel(parcel) => match &parcel.command {
                ParcelCommands::View(ParcelViewCommand { reference }) => via::open(&reference),
            },
        };
    }
}

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct LinearConfig {
    /// Personal Linear API key
    #[clap(long = "linear-api-key", value_parser)]
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<String>,

    #[clap(long = "linear-organization", value_parser)]
    #[serde(skip_serializing_if = "Option::is_none")]
    organization: Option<String>,

    #[clap(long = "linear-use-app", value_parser)]
    #[serde(skip_serializing_if = "Option::is_none")]
    use_app: Option<bool>,
}

#[derive(Subcommand, Serialize, Deserialize, Debug)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Issues(IssuesCommand),

    #[command(arg_required_else_help = true)]
    Issue(IssueCommand),

    #[command(arg_required_else_help = true)]
    Parcel(ParcelCommand),
}

#[derive(Args, Serialize, Deserialize, Debug)]
struct IssueCommand {
    #[command(subcommand)]
    command: IssueCommands,
}

#[derive(Args, Serialize, Deserialize, Debug)]
struct IssuesCommand {}

#[derive(Subcommand, Serialize, Deserialize, Debug)]
enum IssueCommands {
    #[command(arg_required_else_help = true)]
    View(IssueViewCommand),
}

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct IssueViewCommand {
    issue: String,
}

impl IssueViewCommand {
    pub fn run(&self, config: &Cli) {
        let client = LinearClient::new(config.linear.use_app.unwrap_or(false));
        if let Some(organization) = &config.linear.organization {
            client.view(organization, &self.issue);
        } else {
            // TODO: Proper error handling
            eprintln!("Missing organization");
        }
    }
}

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct ParcelCommand {
    #[command(subcommand)]
    command: ParcelCommands,
}

#[derive(Subcommand, Serialize, Deserialize, Debug)]
enum ParcelCommands {
    #[command(arg_required_else_help = true)]
    View(ParcelViewCommand),
}

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct ParcelViewCommand {
    reference: String,
}
