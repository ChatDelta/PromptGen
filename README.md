# PromptGen

PromptGen is a simple command-line tool written in Rust that helps you build multi-line prompts interactively. You can enter your prompt text line by line and then save the final result to a file for later use.

## Features

- Interactive CLI that reads lines of text until you submit an empty line
- Saves your prompt into a file called `my.prompt` in the current directory
- Small codebase built with [`clap`](https://crates.io/crates/clap) for argument parsing and [`anyhow`](https://crates.io/crates/anyhow) for error handling

## Building and Running

1. Install [Rust and Cargo](https://www.rust-lang.org/tools/install) if you have not already.
2. Clone this repository and navigate into the project directory.
3. Run `cargo run` to start the prompt generator:

```bash
cargo run
```

You will see a welcome message followed by a `>` prompt. Type each line of your prompt and press `Enter`. When you're done, press `Enter` on an empty line to write the contents to `my.prompt`.

```
$ cargo run
Welcome to PromptGen!
Enter your prompt line by line. Press ENTER on an empty line to finish.

> This is the first line
> And this is the second line
> 

Prompt saved to my.prompt
```

The resulting `my.prompt` file will contain the text you entered.

## License

PromptGen is distributed under the terms of the MIT license. See [LICENSE](LICENSE) for details.

## Contributing

Feel free to open issues or submit pull requests if you find bugs or have suggestions for improvements.
