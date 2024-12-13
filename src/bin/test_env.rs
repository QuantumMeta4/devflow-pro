use std::env;
use std::fs;

fn main() {
    // Load environment variables from .env file if it exists
    if let Ok(path) = env::var("ENV_FILE") {
        println!("Loading environment from: {}", path);
        match fs::read_to_string(path) {
            Ok(contents) => {
                for line in contents.lines() {
                    if let Some((key, value)) = line.split_once('=') {
                        env::set_var(key.trim(), value.trim());
                    }
                }
            }
            Err(e) => eprintln!("Error reading env file: {}", e),
        }
    }

    // Print all environment variables for debugging
    for (key, value) in env::vars() {
        println!("{}={}", key, value);
    }

    match env::var("TOGETHER_API_KEY") {
        Ok(key) => println!("API key loaded successfully: {key}"),
        Err(e) => println!("Error loading API key: {e}"),
    }
}
