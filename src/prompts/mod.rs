use std::collections::HashMap;

use color_eyre::eyre::Result;

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
