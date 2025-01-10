use gemini_rs::Conversation;
use git2::Repository;
use std::env;

async fn generate_commit_message(
    diff: &str,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut convo = Conversation::new(api_key.to_string(), "gemini-1.5-flash".to_string());

    let prompt = format!("Generate commit message from diff: {}", diff);
    let response = convo.prompt(&prompt).await;
    Ok(response.trim().to_string())
}

fn get_diff(repo_path: &str) -> Result<String, git2::Error> {
    let repo = Repository::open(repo_path)?;
    let head = match repo.head() {
        Ok(head) => head,
        Err(e) => {
            if e.code() == git2::ErrorCode::UnbornBranch {
                return Ok(String::from("No commits in the repository"));
            } else {
                return Err(e);
            }
        }
    };
    let head_commit = head.peel_to_commit()?;
    let tree = head_commit.tree()?;

    let diff = repo.diff_tree_to_workdir_with_index(Some(&tree), None)?;

    let mut diff_str = String::new();

    diff.print(git2::DiffFormat::Patch, |_, _, line| {
        diff_str.push_str(std::str::from_utf8(line.content()).unwrap());
        true
    })?;

    Ok(diff_str)
}
#[tokio::main]
async fn main() {
    let repo_path = ".";
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    match get_diff(repo_path) {
        Ok(diff) => match generate_commit_message(&diff, &api_key).await {
            Ok(commit_message) => println!("Commit Message: {}", commit_message),
            Err(e) => eprintln!("Failed to generate commit message: {}", e),
        },

        Err(e) => println!("Error: {}", e),
    }
    println!("")
}
