use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    match env::var("TOGETHER_API_KEY") {
        Ok(key) => println!("API key found with length: {}", key.len()),
        Err(e) => println!("Error loading API key: {}", e),
    }
}
