#[macro_use]
extern crate log;
use clap::Parser;
use gh_gpt::cli::commands::{Cli, Commands};
use gh_gpt::config::Config;
use gh_gpt::labels::labelize;

fn main() {
    // Initialize the logger.
    env_logger::init();
    info!("hello from gh-gpt 🤖 !");

    // Parse the command line arguments.
    let cli = Cli::parse();

    // Retrieve the application configuration.
    let cfg = config();

    if let Some(command) = cli.command {
        match command {
            Commands::Labelize { gh_issue_number } => {
                labelize(&cfg, gh_issue_number);
            }
        }
    }
}

/// Parse and return the application configuration.
fn config() -> Config {
    // TODO: Parse the configuration from the environment variables.
    Config::default()
}
