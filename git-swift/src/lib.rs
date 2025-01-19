use std::collections::HashMap;

use constants::SERVER_URL;
use serde::Deserialize;

pub mod ai;
pub mod cli;
pub mod git;
pub mod utils;
pub mod constants;

#[derive(Debug, Deserialize)]
pub struct ApiResponse{
    pub messages: Vec<String>
}

pub async fn fetch_commit_messages(diff: &str)->Result<ApiResponse, Box<dyn std::error::Error>>{
    let mut map = HashMap::new();
    map.insert("diff", diff);
    let client = reqwest::Client::new();
    let resp = client.post(format!("{}/commit_message", SERVER_URL)).json(&map).send().await?;
    let body: ApiResponse = resp.json().await?;
    Ok(body)
}