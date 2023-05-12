use std::collections::HashMap;

use async_openai::types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role};
use async_openai::Client;
use color_eyre::eyre::Result;
use log::info;

use crate::config::GhGptConfig;

pub const DEFAULT_MAX_TOKENS: u16 = 4096u16;
pub const DEFAULT_MODEL: &str = "gpt-4";

/// Build a prompt from a template.
/// # Arguments
/// * `template` - The template.
/// * `args` - The arguments, represented as a map of key/value pairs.
pub fn build_prompt(template: &str, args: &HashMap<String, String>) -> Result<String> {
    let mut result = template.to_string();
    for (key, value) in args {
        let token = format!("{{{key}}}");
        result = result.replace(&token, value);
    }
    Ok(result)
}

/// Request a completion from the chatgpt API.
/// # Arguments
/// * `cfg` - The application configuration.
/// * `prompt` - The prompt to send to the API.
/// # Returns
/// The response from the API.
pub async fn chatgpt_request(cfg: &GhGptConfig, prompt: &str) -> Result<String> {
    info!("📜 sending prompt to chatgpt: \n{}", prompt);
    // Create a client.
    let client = Client::new().with_api_key(cfg.openai_api_key.clone());

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(cfg.chatgpt_max_tokens.unwrap_or(DEFAULT_MAX_TOKENS))
        .model(cfg.chatgpt_model.clone().unwrap_or(DEFAULT_MODEL.to_owned()))
        .messages([ChatCompletionRequestMessageArgs::default().role(Role::System).content(prompt).build()?])
        .build()?;

    let response = client.chat().create(request).await?;

    info!("📢 chatgpt response:\n");
    let mut chatgpt_response = String::new();
    for choice in response.choices {
        chatgpt_response.push_str(&format!("{}\n", choice.message.content));
        println!("{}", choice.message.content);
    }
    Ok(chatgpt_response)
}
