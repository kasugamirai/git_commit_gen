# Auto Git Commit

This project is a tool that uses the OpenAI GPT model to automatically generate commit messages for Git commits based on the changes made in the code. It is written in Rust and uses the Tokio runtime for asynchronous operations.

## Setup

1. Clone the repository.
2. `sh build.sh`
3. Set the `OPENAI_API_KEY` environment variable to your OpenAI API key.
4. Run the program with the `generate` flag to generate a commit message.

## Usage

```bash
cp ~/projects/auto_git_commit/output/git_gen /usr/local/bin/
```

## Run

```bash
gitgen -g
```