//! Module related to Github labels.

use log::debug;

use crate::config::Config;

/// Labelize a Github issue.
/// # Arguments
/// * `cfg` - The application configuration.
/// * `gh_issue_number` - The Github issue number.
pub fn labelize(_cfg: &Config, gh_issue_number: u64) {
    debug!("labelize: {}", gh_issue_number);
}
