use std::env;

use axum::{routing::post, Json, Router};
use git_swift::ai::generate_commit_messages;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::errors::Result;

pub fn route() -> Router {
    Router::new().route("/commit_message", post(generate_commit_message))
}

async fn generate_commit_message(payload: Json<RequestPayload>) -> Result<Json<Value>> {
    let api_key = env::var("GEMINI_API_KEY").expect("API key not found.");
    let commit_messages = generate_commit_messages(&payload.diff, &api_key).await;
    match commit_messages{
        Ok(messages) => Ok(Json(json!({ "messages": messages }))),
        Err(_) => Err(crate::errors::Error::UnableToGenerateCommitMessage),
    }
}

#[derive(Debug, Deserialize)]
struct RequestPayload{
    diff: String,
}