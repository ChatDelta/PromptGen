use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

/// Simple interactive prompt generator
#[derive(Parser)]
#[command(author, version, about = "Interactive prompt generator", long_about = None)]
struct Cli {
    /// Analyze and label the tone of the prompt
    #[arg(short, long)]
    tone: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

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

    if cli.tone {
        let tones = analyze_tone(&prompt);
        if !tones.is_empty() {
            println!("\nDetected tone(s): {}", tones.join(", "));
            print!("Would you like to add these tone labels to the filename? (y/n): ");
            io::stdout().flush()?;

            let mut response = String::new();
            io::stdin().read_line(&mut response)?;

            let path = if response.trim().to_lowercase() == "y" {
                let tone_suffix = tones.join("-");
                PathBuf::from(format!("my-{}.prompt", tone_suffix))
            } else {
                PathBuf::from("my.prompt")
            };

            let mut file = File::create(&path)
                .with_context(|| format!("failed to create {}", path.display()))?;
            file.write_all(prompt.as_bytes())?;
            println!("\nPrompt saved to {}", path.display());
        } else {
            println!("\nNo specific tone detected.");
            let path = PathBuf::from("my.prompt");
            let mut file = File::create(&path)
                .with_context(|| format!("failed to create {}", path.display()))?;
            file.write_all(prompt.as_bytes())?;
            println!("\nPrompt saved to {}", path.display());
        }
    } else {
        let path = PathBuf::from("my.prompt");
        let mut file =
            File::create(&path).with_context(|| format!("failed to create {}", path.display()))?;
        file.write_all(prompt.as_bytes())?;
        println!("\nPrompt saved to {}", path.display());
    }

    Ok(())
}

fn analyze_tone(text: &str) -> Vec<String> {
    let mut tone_scores: HashMap<String, i32> = HashMap::new();
    let lower_text = text.to_lowercase();

    // Professional/Formal tone indicators
    let professional_words = [
        "please",
        "kindly",
        "regarding",
        "therefore",
        "furthermore",
        "accordingly",
        "pursuant",
        "hereby",
        "whereas",
        "respectively",
        "sincerely",
        "cordially",
        "shall",
        "ought",
        "endeavor",
    ];

    // Casual/Informal tone indicators
    let casual_words = [
        "hey", "hi", "yeah", "yep", "nope", "gonna", "wanna", "cool", "awesome", "stuff", "things",
        "ok", "okay", "thanks", "btw", "fyi", "lol", "omg",
    ];

    // Technical tone indicators
    let technical_words = [
        "implement",
        "algorithm",
        "function",
        "parameter",
        "variable",
        "database",
        "api",
        "framework",
        "debug",
        "compile",
        "execute",
        "optimize",
        "refactor",
        "deploy",
        "architecture",
        "interface",
        "repository",
        "documentation",
        "configuration",
        "integration",
    ];

    // Creative tone indicators
    let creative_words = [
        "imagine",
        "create",
        "design",
        "invent",
        "inspire",
        "dream",
        "vision",
        "artistic",
        "unique",
        "original",
        "innovative",
        "brainstorm",
        "explore",
        "discover",
        "transform",
    ];

    // Instructional tone indicators
    let instructional_words = [
        "explain",
        "describe",
        "teach",
        "show",
        "demonstrate",
        "guide",
        "help",
        "assist",
        "clarify",
        "elaborate",
        "step",
        "first",
        "next",
        "then",
        "finally",
        "how to",
    ];

    // Analytical tone indicators
    let analytical_words = [
        "analyze",
        "compare",
        "contrast",
        "evaluate",
        "assess",
        "examine",
        "investigate",
        "research",
        "study",
        "consider",
        "review",
        "critique",
        "measure",
        "calculate",
        "determine",
    ];

    // Count occurrences of tone indicators
    for word in professional_words {
        if lower_text.contains(word) {
            *tone_scores.entry("professional".to_string()).or_insert(0) += 1;
        }
    }

    for word in casual_words {
        if lower_text.contains(word) {
            *tone_scores.entry("casual".to_string()).or_insert(0) += 1;
        }
    }

    for word in technical_words {
        if lower_text.contains(word) {
            *tone_scores.entry("technical".to_string()).or_insert(0) += 1;
        }
    }

    for word in creative_words {
        if lower_text.contains(word) {
            *tone_scores.entry("creative".to_string()).or_insert(0) += 1;
        }
    }

    for word in instructional_words {
        if lower_text.contains(word) {
            *tone_scores.entry("instructional".to_string()).or_insert(0) += 1;
        }
    }

    for word in analytical_words {
        if lower_text.contains(word) {
            *tone_scores.entry("analytical".to_string()).or_insert(0) += 1;
        }
    }

    // Check for questions (interrogative tone)
    if text.contains('?') {
        *tone_scores.entry("interrogative".to_string()).or_insert(0) += 2;
    }

    // Check for urgency
    let urgent_words = ["urgent", "asap", "immediately", "now", "quickly", "hurry"];
    for word in urgent_words {
        if lower_text.contains(word) {
            *tone_scores.entry("urgent".to_string()).or_insert(0) += 2;
        }
    }

    // Select tones with significant scores
    let mut detected_tones: Vec<(String, i32)> = tone_scores
        .into_iter()
        .filter(|(_, score)| *score >= 2)
        .collect();

    // Sort by score (highest first)
    detected_tones.sort_by(|a, b| b.1.cmp(&a.1));

    // Return top 3 tones if multiple are detected
    detected_tones
        .into_iter()
        .take(3)
        .map(|(tone, _)| tone)
        .collect()
}
