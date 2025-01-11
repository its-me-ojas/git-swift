use dotenv::dotenv;
use std::env;

pub struct Config {
    pub api_key: String,
    pub repo_path: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        let api_key = env::var("GEMINI_API_KEY")?;
        Ok(Config {
            api_key,
            repo_path: String::from("."),
        })
    }
}
