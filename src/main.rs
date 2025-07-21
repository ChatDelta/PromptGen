use std::fs::File;
use std::io::{self, Write, BufRead};
use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

/// Simple interactive prompt generator
#[derive(Parser)]
#[command(author, version, about = "Interactive prompt generator", long_about = None)]
struct Cli {
    /// File to read the prompt from
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// File to write the prompt to
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Model to use for the prompt
    #[arg(short, long)]
    model: Option<String>,

    /// Mode to use for the prompt
    #[arg(long)]
    mode: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let cli = Cli::parse();

    let mut lines = Vec::new();
    if let Some(path) = &cli.input {
        let file = File::open(path).with_context(|| format!("failed to open {}", path.display()))?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            lines.push(line?);
        }
    } else {
        println!("Welcome to PromptGen!");
        println!("Enter your prompt line by line. Press ENTER on an empty line to finish.\n");

        let stdin = io::stdin();
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
    }

    let mut prompt = lines.join("");

    if let Some(ref model) = cli.model {
        prompt = format!("---\nmodel: {}\n---\n{}", model, prompt);
    }

    if let Some(ref mode) = cli.mode {
        prompt = format!("---\nmode: {}\n---\n{}", mode, prompt);
    }

    let path = cli.output.unwrap_or_else(|| PathBuf::from("my.prompt"));
    let mut file = File::create(&path).with_context(|| format!("failed to create {}", path.display()))?;
    file.write_all(prompt.as_bytes())?;

    println!("\nPrompt saved to {}", path.display());
    if cli.model.is_some() {
        println!("Model: {}", cli.model.unwrap());
    }
    if cli.mode.is_some() {
        println!("Mode: {}", cli.mode.unwrap());
    }
    Ok(())
}
