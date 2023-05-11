//! Module related to Github labels.

use color_eyre::eyre::Result;
use log::info;

use crate::config::GhGptConfig;

/// Labelize a Github issue.
/// # Arguments
/// * `cfg` - The application configuration.
/// * `issue_number` - The Github issue number.
pub async fn labelize(_cfg: &GhGptConfig, org: &str, repo: &str, issue_number: u64) -> Result<()> {
    info!("👨‍💻 starting task: labelize {}/{} issue #{}", org, repo, issue_number);
    // Retrieve the issue.
    let issue = octocrab::instance().issues(org, repo).get(issue_number).await?;
    info!("issue title: {:?}", issue.title);

    // Update the issue.
    // TODO: query the GPT API to retrieve the suggested labels.
    octocrab::instance().issues(org, repo).update(issue_number).labels(&["gh-gpt-bot".to_owned()]).send().await?;

    info!("✅ task completed: labelize {}/{} issue #{}", org, repo, issue_number);
    Ok(())
}
