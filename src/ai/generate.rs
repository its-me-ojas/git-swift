use gemini_rs::Conversation;

pub async fn generate_commit_message(
    diff: &str,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut convo = Conversation::new(api_key.to_string(), "gemini-1.5-flash".to_string());

    let prompt = format!(
        "Analyze the following git diff and generate a concise commit message adhering to conventional commit message standards (e.g., 50-character summary line, followed by a detailed body if necessary). Focus primarily on code logic, structure, and refactoring changes, and handle dependency updates as follows:

        1. Highlight changes to the logic and structure of the code, such as new functions, modified algorithms, or reorganized files.
        2. Summarize refactoring efforts, such as moving files, renaming components, or improving readability.
        3. Emphasize new features, bug fixes, or improvements in functionality.
        4. If a major revamp involves integrating or leveraging a dependency (e.g., migrating to 'clap' for CLI parsing), explicitly highlight this in the main message, treating it as a key change to the codebase.
        5. De-emphasize changes to Cargo.lock or similar dependency-related files unless they are the only changes in the diff or critical to understanding the updates.

        Return only the commit message text, formatted properly, without any additional context or explanations.

        Git Diff:
        {}
        ",
        diff
    );

    let response = convo.prompt(&prompt).await;
    Ok(response.trim().to_string())
}

pub async fn generate_commit_messages(
    diff: &str,
    api_key: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut convo = Conversation::new(api_key.to_string(), "gemini-1.5-flash".to_string());

    let prompt = format!(
        "Analyze the following git diff and generate 3 different commit messages, each adhering to conventional commit message standards (e.g., 50-character summary line, followed by a detailed body if necessary).

        Each message should have a different focus/perspective but don't include any headers or labels for the options.
        Just provide the commit messages directly, separated by '---' on a new line.

        Make sure each message follows the conventional commit format and is ready to be used directly.

        Git Diff:
        {}
        ",
        diff
    );

    let response = convo.prompt(&prompt).await;
    let messages: Vec<String> = response
        .split("---")
        .map(|msg| msg.trim().to_string())
        .filter(|msg| !msg.is_empty())
        .collect();

    Ok(messages)
}
