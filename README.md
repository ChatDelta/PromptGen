# PromptGen

A flexible and powerful prompt generator in Rust.

## Features

- **Configuration-driven:** Define your prompts in a `promptgen.toml` file for easy management and reuse.
- **Templating:** Use Handlebars templates to create dynamic and complex prompts.
- **Customizable:** Specify models, modes, and output files to tailor the generation process.

## Usage

1.  Create a `promptgen.toml` file in your project directory:

    ```toml
    [prompts.default]
    model = "gpt-3.5-turbo"
    mode = "completion"
    template = "Write a story about {{user_input}}."
    ```

2.  Run PromptGen with the name of the prompt you want to generate:

    ```bash
    promptgen default
    ```

3.  Enter your prompt text, and PromptGen will generate the final prompt based on your configuration and template.
