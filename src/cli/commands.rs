//! Defines the CLI commands.

// Imports
use clap::{Parser, Subcommand};

/// Main CLI struct
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The subcommand to run.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Automatically add relevant labels to a Github issue.
    Labelize { org: String, repo: String, issue_number: u64 },
}
