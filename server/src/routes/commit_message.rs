use std::env;

use axum::{
    http::{HeaderMap, HeaderValue},
    routing::{get, post},
    Json, Router,
};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::errors::Result;

pub fn route() -> Router {
    Router::new()
        .route("/commit_message", post(generate_commit_message))
        .route("/", get(hello_controller))
}

async fn hello_controller() -> Result<Json<Value>> {
    Ok(Json(json!({ "message": "You have Reached the Gitswift Server" })))
}

async fn generate_commit_message(payload: Json<RequestPayload>) -> Result<Json<Value>> {
    let api_key = env::var("GROQ_API_KEY").map_err(|e| {
        eprintln!("Failed to get GROQ_API_KEY: {}", e);
        crate::errors::Error::UnableToGenerateCommitMessage
    })?;
    
    let commit_messages = generate_commit_messages(&payload.diff, &api_key).await;
    match commit_messages {
        Ok(messages) => Ok(Json(json!({ "messages": messages }))),
        Err(e) => {
            eprintln!("Failed to generate commit messages: {:?}", e);
            Err(e)
        }
    }
}

pub async fn generate_commit_messages(diff: &str, api_key: &str) -> Result<Vec<String>> {
    let client = reqwest::Client::new();
    let models = vec![
        "gemma2-9b-it",
        "llama-3.3-70b-versatile",
        "llama-3.1-8b-instant"
    ];
    
    let mut all_messages = Vec::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| {
                eprintln!("Header error: {}", e);
                crate::errors::Error::UnableToGenerateCommitMessage
            })?
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let prompt = format!(
        "
        Git Diff:
        {}
        ",
        diff
    );
    let system_prompt = "You are an AI tool that analyzes git diffs and generates commit messages.
    Generate a single commit message that follows conventional commit message standards (e.g., 50-character summary line, followed by a detailed body if necessary).
    
    Use prefixes like Fix: Refactor: Feat: Chore: Docs: etc. as needed.

    IMPORTANT:
    Make sure the message follows the conventional/standard commit format and is ready to be used directly.
    Don't give any explanation or meta-commentary about the message.
    ";

    for model in models {
        println!("Trying model: {}", model);  // Debug log
        
        let request_body = GroqRequest {
            model: model.to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt.clone(),
                }
            ],
            temperature: 0.5,
        };

        let response = client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .headers(headers.clone())
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                eprintln!("API request error for model {}: {}", model, e);
                crate::errors::Error::UnableToGenerateCommitMessage
            })?;

        let response_text = response
            .text()
            .await
            .map_err(|e| {
                eprintln!("Failed to get response text for model {}: {}", model, e);
                crate::errors::Error::UnableToGenerateCommitMessage
            })?;

        println!("Response from {}: {}", model, response_text);  // Debug log

        let groq_response: GroqResponse = serde_json::from_str(&response_text)
            .map_err(|e| {
                eprintln!("Failed to parse JSON for model {}: {}", model, e);
                eprintln!("Response text: {}", response_text);
                crate::errors::Error::UnableToGenerateCommitMessage
            })?;

        all_messages.push(groq_response.choices[0].message.content.trim().to_string());
    }

    Ok(all_messages)
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
