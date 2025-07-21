use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub prompts: HashMap<String, Prompt>,
}

#[derive(Deserialize)]
pub struct Prompt {
    pub model: Option<String>,
    pub mode: Option<String>,
    pub template: String,
}

pub fn load_config(path: &PathBuf) -> anyhow::Result<Config> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
