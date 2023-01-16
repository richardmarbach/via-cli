use clap::{Args, Subcommand};
use color_eyre::Result;

use crate::{
    api::linear::LinearClient,
    cli::{self, Config},
};

#[derive(Args, Debug)]
pub struct IssuesCommand {
    #[command(subcommand)]
    command: Commands,
}

impl IssuesCommand {
    pub fn run(&self, config: &Config) -> Result<()> {
        match &self.command {
            Commands::Assigned(command) => command.run(&config)
        }
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "List assigned issues")]
    Assigned(AssignedCommand),
}

#[derive(Args, Debug)]
pub struct AssignedCommand { }

impl AssignedCommand {
    pub fn run(&self, config: &Config) -> Result<()> {
        let client = LinearClient::new(&config.linear_api_key);
        let user_id = client.current_user_id()?;

        let issues = client.assigned_issues(&user_id)?;

        match config.format {
            cli::OutputFormat::TEXT => {
                if issues.is_empty() {
                    println!("You don't have any assigned issues!");
                    return Ok(());
                }

                for issue in issues {
                    let number = issue.number;
                    let title = issue.title;
                    let url = issue.url;
                    let description = issue.description.unwrap_or("".to_owned());
                    println!(
                        r#"{number} {title}
{url}

{description}
"#
                    );
                }
                Ok(())
            }
            cli::OutputFormat::JSON => {
                let json = serde_json::to_string(&issues)?;
                println!("{json}");
                Ok(())
            }
        }
    }
}
