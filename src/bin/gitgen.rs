use auto_git_commit::cli;
use auto_git_commit::git;
use auto_git_commit::gpt;
use dotenv::dotenv;
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();
    let matches = cli::commands::build_cli_app().get_matches();
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            log::error!("Please set the OPENAI_API_KEY environment variable, such as export OPENAI_API_KEY=\"your-api\"");
            return;
        }
    };

    let gpt_client = match gpt::msg::GPTClient::new(api_key).await {
        Ok(client) => client,
        Err(e) => {
            log::error!("Failed to create GPT client: {}", e);
            return;
        }
    };

    if matches.get_flag("generate") {
        let prompt = "Please provide a commit message based on the changes, ensuring to wrap the body at 72 characters. Use the body to detail what changes were made and why, rather than the method of implementation.";
        let commit = git::ops::Commit::new();
        let current_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                log::error!("Failed to get current directory: {}", e);
                return;
            }
        };
        let repo_path = current_dir.as_path();
        let git_changes = match commit.read_changes(repo_path) {
            Ok(changes) => changes,
            Err(e) => {
                log::error!("Failed to read changes: {}", e);
                return;
            }
        };

        let git_diff = match commit.get_git_diff(repo_path) {
            Ok(diff) => diff,
            Err(e) => {
                log::error!("Failed to get git diff: {}", e);
                return;
            }
        };
        log::info!("Changes: {}", git_changes);
        log::info!("Diff: {}", git_diff);
        let message = format!("{}\n{}\n{}", prompt, git_changes, git_diff);
        let response = gpt_client.send_message_streaming(&message).await.unwrap();
        print!("\nDo you want to commit? (yes/no): ");
        io::stdout().flush().unwrap();
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();
        if answer.trim() == "yes" {
            commit.git_commit(&response).unwrap();
        }
    }
}
