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

pub fn select_commit_message(messages: &[String]) -> Option<String> {
    println!("\nPlease select a commit message option (1-{}) or 0 to cancel:", messages.len());

    for (i, message) in messages.iter().enumerate() {
        println!("\nOption {}:\n{}", i + 1, message);
    }

    print!("\nYour choice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(n) if n == 0 => None,
        Ok(n) if n <= messages.len() => Some(messages[n - 1].clone()),
        _ => {
            println!("Invalid selection. Operation cancelled.");
            None
        }
    }
}
