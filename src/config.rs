//! General configuration

/// Configuration for the application.
#[derive(Debug, Default)]
pub struct Config {
    pub api_keys: ApiKeys,
}

/// Configuration for the API keys.
#[derive(Debug, Default)]
pub struct ApiKeys {
    pub github_token: String,
    pub openai_api_key: String,
}
