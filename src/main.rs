mod config;

use std::fs::File;
use std::io::{self, Write, BufRead};
use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use handlebars::Handlebars;

/// Simple interactive prompt generator
#[derive(Parser)]
#[command(author, version, about = "Interactive prompt generator", long_about = None)]
struct Cli {
    /// Name of the prompt to generate
    prompt_name: String,

    /// File to write the prompt to
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Path to the configuration file
    #[arg(short, long, default_value = "promptgen.toml")]
    config: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config = config::load_config(&cli.config)?;

    let prompt_config = config
        .prompts
        .get(&cli.prompt_name)
        .context("Prompt not found in config")?;

    println!("Welcome to PromptGen!");
    println!("Enter your prompt line by line. Press ENTER on an empty line to finish.\n");

    let stdin = io::stdin();
    let mut lines = Vec::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.lock().read_line(&mut input)?;
        if input.trim().is_empty() {
            break;
        }
        lines.push(input);
    }

    let mut data = std::collections::HashMap::new();
    data.insert("user_input".to_string(), lines.join(""));

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("prompt", &prompt_config.template)?;

    let prompt = handlebars.render("prompt", &data)?;

    let path = cli
        .output
        .clone()
        .unwrap_or_else(|| PathBuf::from("my.prompt"));
    let mut file = File::create(&path).with_context(|| format!("failed to create {}", path.display()))?;
    file.write_all(prompt.as_bytes())?;

    let path = cli.output.unwrap_or_else(|| PathBuf::from("my.prompt"));
    let mut file = File::create(&path).with_context(|| format!("failed to create {}", path.display()))?;
    file.write_all(prompt.as_bytes())?;

    println!("\nPrompt saved to {}", path.display());
    if let Some(model) = &prompt_config.model {
        println!("Model: {}", model);
    }
    if let Some(mode) = &prompt_config.mode {
        println!("Mode: {}", mode);
    }
    Ok(())
}
