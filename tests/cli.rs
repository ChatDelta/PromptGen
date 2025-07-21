use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_default_prompt() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("PromptGen")?;
    cmd.arg("default");
    let mut child = cmd.stdin(Stdio::piped()).spawn()?;
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all("This is a test prompt.\n".as_bytes())
            .expect("Failed to write to stdin");
    });
    let output = child.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Prompt saved to my.prompt"));
    assert!(stdout.contains("Model: gpt-3.5-turbo"));
    assert!(stdout.contains("Mode: completion"));
    let content = std::fs::read_to_string("my.prompt")?;
    assert_eq!(content.trim(), "This is a test prompt.");
    std::fs::remove_file("my.prompt")?;
    Ok(())
}

#[test]
fn test_custom_output() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("PromptGen")?;
    cmd.arg("default")
        .arg("-o")
        .arg("custom.prompt");
    let mut child = cmd.stdin(Stdio::piped()).spawn()?;
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all("This is a test prompt.\n".as_bytes())
            .expect("Failed to write to stdin");
    });
    let output = child.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Prompt saved to custom.prompt"));
    let content = std::fs::read_to_string("custom.prompt")?;
    assert_eq!(content.trim(), "This is a test prompt.");
    std::fs::remove_file("custom.prompt")?;
    Ok(())
}

#[test]
fn test_missing_config() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("PromptGen")?;
    cmd.arg("default")
        .arg("-c")
        .arg("nonexistent.toml");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}

#[test]
fn test_missing_prompt() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("PromptGen")?;
    cmd.arg("nonexistent");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Prompt not found in config"));
    Ok(())
}
