use std::io::{self, Write};

pub async fn confirm_commit() -> bool {
    print!("Do you want to commit and push these changes? (y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}
