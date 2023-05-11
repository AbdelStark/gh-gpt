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
    Labelize { gh_issue_number: u64 },
}
