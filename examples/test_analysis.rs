use std::fs;
use std::path::PathBuf;

// Function with several code quality and security issues
pub fn process_user_data(user_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Security issue: Directly using unwrap
    let path = PathBuf::from("user_data.txt");
    if !path.exists() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        )));
    }
    let data = fs::read_to_string(path)?;

    // Performance issue: Unnecessary string allocation
    let mut result = String::from("");

    // Security issue: No input validation
    if user_input.len() > 0 {
        // Performance issue: Inefficient string concatenation
        result = result + &data + user_input;
    }

    // Security issue: Writing to file without proper error handling
    let output_path = PathBuf::from("output.txt");
    fs::write(output_path, &result)?;

    // Performance issue: Unnecessary clone
    Ok(result.clone())
}

// Function with potential memory issues
pub fn process_large_data(data: Vec<String>) -> Vec<String> {
    // Performance issue: Multiple allocations
    let mut results = Vec::new();

    for item in data {
        // Memory issue: Growing vector without capacity
        results.push(item.to_uppercase());
    }

    results
}

fn main() {
    let test_input = "test data";
    match process_user_data(test_input) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }

    let test_data = vec!["hello".to_string(), "world".to_string()];
    let processed = process_large_data(test_data);
    println!("Processed: {:?}", processed);
}
