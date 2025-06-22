use std::fs::File;
use std::io::{self, Write, BufRead};
use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

/// Simple interactive prompt generator
#[derive(Parser)]
#[command(author, version, about = "Interactive prompt generator", long_about = None)]
struct Cli {}

fn main() -> anyhow::Result<()> {
    let _cli = Cli::parse();

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

    let prompt = lines.join("");

    let path = PathBuf::from("my.prompt");
    let mut file = File::create(&path).with_context(|| format!("failed to create {}", path.display()))?;
    file.write_all(prompt.as_bytes())?;

    println!("\nPrompt saved to {}", path.display());
    Ok(())
}
