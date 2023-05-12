//! Module related to Github labels.

use std::collections::HashMap;

use color_eyre::eyre::Result;
use log::info;
use octocrab::models::issues::Issue;

use crate::config::GhGptConfig;

// Define constants.
const GH_GPT_BOT: &str = "gh-gpt-bot";
const DEFAULT_MAX_LABELS: &str = "5";

/// Labelize a Github issue.
/// # Arguments
/// * `cfg` - The application configuration.
/// * `issue_number` - The Github issue number.
pub async fn labelize(cfg: &GhGptConfig, org: &str, repo: &str, issue_number: u64) -> Result<()> {
    info!("👨‍💻 starting task: labelize {}/{} issue #{}", org, repo, issue_number);
    // Retrieve the issue.
    let issue = octocrab::instance().issues(org, repo).get(issue_number).await?;
    info!("issue title: {:?}", issue.title);

    // Retrieve the suggested labels.
    let suggested_labels = suggest_labels(cfg, repo, &issue).await?;

    // Update the issue.
    octocrab::instance().issues(org, repo).update(issue_number).labels(&suggested_labels).send().await?;

    info!("✅ task completed: labelize {}/{} issue #{}", org, repo, issue_number);
    Ok(())
}

/// Suggest labels for a Github issue.
/// # Arguments
/// * `cfg` - The application configuration.
/// * `repo` - The Github repository.
/// * `issue` - The Github issue.
async fn suggest_labels(_cfg: &GhGptConfig, repo: &str, issue: &Issue) -> Result<Labels> {
    let labels = vec![GH_GPT_BOT.to_owned()];
    let prompt_template = include_str!("prompts/templates/labelize.txt");
    let args = HashMap::from([
        ("repo_name".to_owned(), repo.to_owned()),
        ("issue_title".to_owned(), issue.title.clone()),
        ("issue_body".to_owned(), issue.body.clone().unwrap_or_default()),
        ("max_labels".to_owned(), DEFAULT_MAX_LABELS.to_owned()),
    ]);
    let prompt = crate::prompts::build_prompt(prompt_template, &args)?;
    info!("prompt: \n{}", prompt);
    Ok(labels)
}

/// A type to represent a Github label.
pub type Label = String;
/// A type to represent a list of Github labels.
pub type Labels = Vec<Label>;
