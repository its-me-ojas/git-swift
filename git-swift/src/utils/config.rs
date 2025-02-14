use std::{env, path::PathBuf};

pub struct Config {
    pub repo_path: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // try loading from multiple .env locations
        let env_locations = vec![
            // current dir
            Some(PathBuf::from(".env")),
            // user's home dir
            dirs::home_dir().map(|mut p| {
                p.push(".git-swift");
                p.push(".env");
                p
            }),
        ];

        // try loading .env from each location
        for path in env_locations.into_iter().flatten() {
            if path.exists() {
                dotenv::from_path(path).ok();
            }
        }

        Ok(Config {
            repo_path: String::from("../../test-repo"),
        })
    }
}
