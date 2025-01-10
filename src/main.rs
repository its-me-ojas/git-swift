use dotenv::dotenv;
use gemini_rs::Conversation;
use git2::Repository;
use std::env;
use std::io;
use std::io::Write;
use tokio::process::Command;

async fn generate_commit_message(
    diff: &str,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut convo = Conversation::new(api_key.to_string(), "gemini-1.5-flash".to_string());

    let prompt = format!(
           "Given the following git diff, generate a concise and meaningful commit message following conventional commit standards.

           Focus primarily on the logic and structural changes to the code, and briefly acknowledge any modifications to dependencies only if relevant. Ensure the commit message is clear, concise, and does not exceed 72 characters per line, following proper commit message conventions.

           Here is the git diff:

           {}

           Return only the commit message as output, nothing else.",
           diff
       );
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

async fn commit_and_push(commit_message: &str) -> Result<(), Box<dyn std::error::Error>> {
    // run 'git add .'
    let status = Command::new("git").arg("add").arg(".").status().await?;
    if !status.success() {
        return Err("Failed to add changes".into());
    }

    // run 'git commit -m <commit_message>'
    let status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .status()
        .await?;
    if !status.success() {
        return Err("Failed to commit changes".into());
    }

    // run 'git push'
    let status = Command::new("git").arg("push").status().await?;
    if !status.success() {
        return Err("Failed to push changes".into());
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let repo_path = ".";
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    match get_diff(repo_path) {
        Ok(diff) => match generate_commit_message(&diff, &api_key).await {
            Ok(commit_message) => {
                println!("Commit Message: {}", commit_message);

                // ask user to confirm commit message
                print!("Do you want to commit and push these changes? (y/n):");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                match input.trim().to_lowercase().as_str() {
                    "y" | "yes" => match commit_and_push(&commit_message).await {
                        Ok(_) => println!("Changes committed and pushed successfully"),
                        Err(e) => eprintln!("Failed to commit and push changes: {}", e),
                    },
                    _ => println!("Operation cancelled by user"),
                }
            }
            Err(e) => eprintln!("Failed to generate commit message: {}", e),
        },

        Err(e) => println!("Error: {}", e),
    }
    println!("")
}
