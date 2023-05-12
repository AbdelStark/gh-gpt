//! General configuration

use color_eyre::eyre::Result;
use config::Config;
use serde_derive::Deserialize;

/// Configuration for the application.
#[derive(Debug, Default, Deserialize)]
pub struct GhGptConfig {
    pub github_token: String,
    pub openai_api_key: String,
    pub chatgpt_model: Option<String>,
    pub chatgpt_max_tokens: Option<u16>,
}

impl GhGptConfig {
    /// Create a new configuration from environment variables.
    pub fn new() -> Result<Self> {
        CONFIG.clone().try_deserialize().map_err(|e| e.into())
    }
}

lazy_static::lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Config = Config::builder()
        .add_source(config::Environment::with_prefix("gh_gpt"))
        .build()
        .unwrap();
}
