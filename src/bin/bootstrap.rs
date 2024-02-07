use dotenv::dotenv;
use std::env;
use std::io::{self, Write};

use auto_git_commit::cli;
use auto_git_commit::git;
use auto_git_commit::gpt;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let matches = cli::commands::build_cli_app().get_matches();
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Please set the OPENAI_API_KEY environment variable");
            return;
        }
    };

    let gpt_client = match gpt::msg::GPTClient::new(api_key).await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create GPT client: {}", e);
            return;
        }
    };

    if matches.get_flag("generate") {
        let prompt = "This GPT is designed to assist with automatically creating commit messages for Git commits based on the provided changes. It follows specific formatting guidelines for the subject and body of the commit message, including separating the subject from the body with a blank line, limiting the subject line to 50 characters, capitalizing the subject line, avoiding ending the subject line with a period, using the imperative mood in the subject line, and wrapping the body at 72 characters. The body of the commit message should explain what and why the changes were made, rather than how they were implemented. Please based on below changes, provide a commit message: ";
        let commit = git::ops::Commit::new();
        let current_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("Failed to get current directory: {}", e);
                return;
            }
        };
        let repo_path = current_dir.as_path();
        let git_changes = match commit.read_changes(repo_path) {
            Ok(changes) => changes,
            Err(e) => {
                eprintln!("Failed to read changes: {}", e);
                return;
            }
        };
        let message = format!("{}\n{}", git_changes, prompt);
        let response = gpt_client.send_message_streaming(&message).await.unwrap();
        print!("Do you want to push? (yes/no): ");
        io::stdout().flush().unwrap();
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();
        if answer.trim() == "yes" {
            commit.git_commit(&response).unwrap();
        }
    }
}
