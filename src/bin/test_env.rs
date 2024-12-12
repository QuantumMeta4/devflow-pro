use dotenv::dotenv;

fn main() {
    dotenv().ok();
    match dotenv::var("TOGETHER_API_KEY") {
        Ok(key) => println!("API key loaded successfully: {key}"),
        Err(e) => println!("Error loading API key: {e}"),
    }
}
