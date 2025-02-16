use std::env;

use axum::{routing::post, Json, Router};
use gemini_rs::Conversation;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::errors::Result;

pub fn route() -> Router {
    Router::new().route("/commit_message", post(generate_commit_message))
}

async fn generate_commit_message(payload: Json<RequestPayload>) -> Result<Json<Value>> {
    println!("Called");
    let api_key = env::var("GEMINI_API_KEY").expect("API key not found.");
    let commit_messages = generate_commit_messages(&payload.diff, &api_key).await;
    match commit_messages {
        Ok(messages) => Ok(Json(json!({ "messages": messages }))),
        Err(_) => Err(crate::errors::Error::UnableToGenerateCommitMessage),
    }
}

pub async fn generate_commit_messages(diff: &str, api_key: &str) -> Result<Vec<String>> {
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

#[derive(Debug, Deserialize)]
struct RequestPayload {
    diff: String,
}
