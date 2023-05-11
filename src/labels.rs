//! Module related to Github labels.

use color_eyre::eyre::Result;
use log::debug;

use crate::config::GhGptConfig;

/// Labelize a Github issue.
/// # Arguments
/// * `cfg` - The application configuration.
/// * `gh_issue_number` - The Github issue number.
pub fn labelize(_cfg: &GhGptConfig, gh_issue_number: u64) -> Result<()> {
    debug!("labelize: {}", gh_issue_number);
    Ok(())
}
