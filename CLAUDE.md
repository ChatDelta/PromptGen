# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

PromptGen is a Rust CLI tool for creating multi-line prompts interactively. The entire application logic is contained in a single file: `src/main.rs`.

## Essential Commands

### Build and Run
```bash
cargo run        # Build and run the application
cargo build      # Build the project
cargo build --release  # Build optimized release version
```

### Development Commands
```bash
cargo check      # Quick compilation check without building
cargo clippy     # Run Rust linter for code quality
cargo fmt        # Format code according to Rust standards
```

### Testing
```bash
cargo test       # Run tests (note: no tests currently exist)
```

## Architecture

The application follows a simple single-file architecture:

- **Entry Point**: `src/main.rs` contains all application logic
- **Core Flow**: 
  1. CLI struct defined with `clap::Parser` (ready for future arguments)
  2. Interactive loop reads user input line by line
  3. Concatenates lines until empty line entered
  4. Saves to `my.prompt` file in current directory
- **Error Handling**: Uses `anyhow` for robust error management throughout

## Key Implementation Details

- The output file `my.prompt` is hardcoded in the application
- Input reading continues until an empty line is entered
- All lines are joined with newline characters preserving multi-line structure
- File writing uses proper error handling with contextual messages

## Dependencies

- `clap` (4.5): Command-line argument parsing framework
- `anyhow` (1.x): Error handling and context propagation

## Development Notes

When modifying this codebase:
- Maintain the simple, focused nature of the tool
- Follow Rust idioms and use `cargo clippy` before committing
- Use `anyhow::Result` for error handling consistency
- The CLI struct is prepared for future argument additions if needed