use git_swift::{
    ai::generate_commit_message,
    cli::confirm_commit,
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

    let commit_message = match generate_commit_message(&diff, &config.api_key).await {
        Ok(msg) => msg,
        Err(e) => {
            eprintln!("Failed to generate commit message: {}", e);
            return;
        }
    };

    println!("Commit Message: {}", commit_message);

    if confirm_commit().await {
        match commit_and_push(&commit_message).await {
            Ok(_) => println!("Changes committed and pushed successfully"),
            Err(e) => eprintln!("Failed to commit and push changes: {}", e),
        }
    } else {
        println!("Operation cancelled by user");
    }
}
