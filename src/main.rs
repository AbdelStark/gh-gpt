#[macro_use]
extern crate log;
use clap::Parser;
use color_eyre::eyre::Result;
use dotenv::dotenv;
use gh_gpt::cli::commands::{Cli, Commands};
use gh_gpt::config::GhGptConfig;
use gh_gpt::labels::labelize;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger.
    env_logger::init();

    // Initialize the error handler.
    color_eyre::install()?;

    // Load the environment variables from the .env file.
    dotenv().ok();

    // Retrieve the application configuration.
    let cfg = GhGptConfig::new()?;

    // Initialize the Github client API.
    init_github_client_api(&cfg)?;

    // Say hello.
    info!("hello from gh-gpt 🤖 !");

    // Parse the command line arguments.
    let cli = Cli::parse();

    // Execute the command.
    match cli.command {
        Some(command) => match command {
            Commands::Labelize { issue_number, org, repo } => labelize(&cfg, &org, &repo, issue_number).await?,
        },
        None => {
            info!("nothing to do there, bye 👋");
        }
    }
    Ok(())
}

/// Initialize the Github client API.
fn init_github_client_api(cfg: &GhGptConfig) -> Result<()> {
    // Initialises the static instance with configuration so that it can be statically accessed
    // everywhere.
    octocrab::initialise(octocrab::Octocrab::builder().personal_token(cfg.github_token.clone()).build()?);
    Ok(())
}
