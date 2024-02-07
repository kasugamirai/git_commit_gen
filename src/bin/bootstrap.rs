use dotenv::dotenv;
use std::env;

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

    if matches.contains_id("generate") {
        let prompt = "Summarize the changes made for a Git commit:";
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
        println!("Generated message: {}", message);
        let response = gpt_client.send_message_streaming(&message).await.unwrap();
        commit.git_commit(&response).unwrap();
    }
}
