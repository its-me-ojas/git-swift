use git_swift::{
    ai::generate_commit_messages,
    cli::{confirm_commit, select_commit_message},
    git::{commit_and_push, get_diff},
    utils::Config,
};

#[tokio::main]
async fn main() {
    let config = Config::new().expect("Failed to load configuration");

    let diff = match get_diff(&config.repo_path) {
        Ok(diff) => diff,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let commit_messages = match generate_commit_messages(&diff, &config.api_key).await {
        Ok(msgs) => msgs,
        Err(e) => {
            eprintln!("Failed to generate commit messages: {}", e);
            return;
        }
    };

    let selected_message = match select_commit_message(&commit_messages) {
        Some(msg) => msg,
        None => {
            println!("No commit message selected. Operation cancelled.");
            return;
        }
    };

    if confirm_commit().await {
        match commit_and_push(&selected_message).await {
            Ok(_) => println!("Changes committed and pushed successfully"),
            Err(e) => eprintln!("Failed to commit and push changes: {}", e),
        }
    } else {
        println!("Operation cancelled by user");
    }
}
