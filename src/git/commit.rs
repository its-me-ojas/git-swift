use tokio::process::Command;

pub async fn commit_and_push(commit_message: &str) -> Result<(), Box<dyn std::error::Error>> {
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
