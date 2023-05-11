#[macro_use]
extern crate log;
use clap::Parser;
use color_eyre::eyre::Result;
use dotenv::dotenv;
use gh_gpt::cli::commands::{Cli, Commands};
use gh_gpt::config::GhGptConfig;
use gh_gpt::labels::labelize;

fn main() -> Result<()> {
    // Initialize the logger.
    env_logger::init();

    // Initialize the error handler.
    color_eyre::install()?;

    // Load the environment variables from the .env file.
    dotenv()?;

    // Say hello.
    info!("hello from gh-gpt 🤖 !");

    // Parse the command line arguments.
    let cli = Cli::parse();

    // Retrieve the application configuration.
    let cfg = GhGptConfig::new()?;

    // Execute the command.
    match cli.command {
        Some(command) => match command {
            Commands::Labelize { gh_issue_number } => labelize(&cfg, gh_issue_number),
        },
        None => {
            info!("nothing to do there, bye 👋");
            Ok(())
        }
    }
}
