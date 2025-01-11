use std::{env, path::PathBuf};

pub struct Config {
    pub api_key: String,
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

        // first try to get API key from environment
        let api_key = match env::var("GEMINI_API_KEY") {
            Ok(key) => key,
            Err(_) => {
                // if not found in environment, try to read from config file
                let config_path = dirs::home_dir()
                    .ok_or("Could not find home directory")?
                    .join(".git-swift")
                    .join("config");

                if config_path.exists() {
                    std::fs::read_to_string(config_path)?
                } else {
                    return Err(
                        "GEMINI_API_KEY not found in environment or config file. Please set it up."
                            .into(),
                    );
                }
            }
        };

        Ok(Config {
            api_key,
            repo_path: String::from("."),
        })
    }

    pub fn setup(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
        // create .git-swift dir
        let config_dir = dirs::home_dir()
            .ok_or("Could not find home directory")?
            .join(".git-swift");

        std::fs::create_dir_all(&config_dir)?;

        // save API KEY to config file
        let config_path = config_dir.join("config");
        std::fs::write(config_path, api_key)?;

        Ok(())
    }
}
