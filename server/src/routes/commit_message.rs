use std::env;

use axum::{
    http::{HeaderMap, HeaderValue},
    routing::post,
    Json, Router,
};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::errors::Result;

pub fn route() -> Router {
    Router::new().route("/commit_message", post(generate_commit_message))
}

async fn generate_commit_message(payload: Json<RequestPayload>) -> Result<Json<Value>> {
    println!("Called");
    let api_key = env::var("GROQ_API_KEY").expect("API key not found.");
    let commit_messages = generate_commit_messages(&payload.diff, &api_key).await;
    match commit_messages {
        Ok(messages) => Ok(Json(json!({ "messages": messages }))),
        Err(_) => Err(crate::errors::Error::UnableToGenerateCommitMessage),
    }
}

pub async fn generate_commit_messages(diff: &str, api_key: &str) -> Result<Vec<String>> {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let prompt = format!(
        "Analyze the following git diff and generate 3 different commit messages, each adhering to conventional commit message standards (e.g., 50-character summary line, followed by a detailed body if necessary).

        Each message should have a different focus/perspective but don't include any headers or labels for the options.
        Just provide the commit messages directly, separated by '---' on a new line.

        IMPORTANT:
        Make sure each message follows the conventional/standard commit format and is ready to be used directly.

        Git Diff:
        {}
        ",
        diff
    );

    let request_body = GroqRequest {
        model: "llama-3.1-8b-instant".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: 0.5,
    };

    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .headers(headers)
        .json(&request_body)
        .send()
        .await
        .map_err(|_| crate::errors::Error::UnableToGenerateCommitMessage)?;

    let response_text = response
        .text()
        .await
        .map_err(|_| crate::errors::Error::UnableToGenerateCommitMessage)?;

    println!("Response from Groq: {}", response_text);

    let groq_response: GroqResponse = serde_json::from_str(&response_text)
        .map_err(|_| crate::errors::Error::UnableToGenerateCommitMessage)?;

    let messages: Vec<String> = groq_response.choices[0]
        .message
        .content
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

#[derive(Debug, Serialize)]
struct GroqRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct GroqResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}
