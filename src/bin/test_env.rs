use std::env;
use std::fs;

fn main() {
    // Load environment variables from .env file if it exists
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let path = &args[1];
        println!("Loading environment from: {path}");
        match fs::read_to_string(path) {
            Ok(contents) => {
                for line in contents.lines() {
                    if line.trim().is_empty() || line.starts_with('#') {
                        continue;
                    }
                    process_env_line(line);
                }
            }
            Err(e) => eprintln!("Error reading env file: {e}"),
        }
    }

    // Print all environment variables for debugging
    for (key, value) in env::vars() {
        println!("{key}={value}");
    }

    match env::var("TOGETHER_API_KEY") {
        Ok(key) => println!("API key loaded successfully: {key}"),
        Err(e) => println!("Error loading API key: {e}"),
    }
}

fn process_env_line(line: &str) {
    if let Some((key, value)) = line.split_once('=') {
        env::set_var(key.trim(), value.trim());
    }
}
