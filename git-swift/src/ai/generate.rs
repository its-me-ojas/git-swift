use crate::constants::SERVER_URL;
use serde::Deserialize;
use std::collections::HashMap;

pub async fn fetch_commit_messages(diff: &str) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("diff", diff);
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/commit_message", SERVER_URL))
        .json(&map)
        .send()
        .await?;
    let body: ApiResponse = resp.json().await?;
    Ok(body)
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub messages: Vec<String>,
}
