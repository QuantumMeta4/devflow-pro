use anyhow::{Context, Result};
use std::path::PathBuf;

/// Process user data and return a formatted string.
///
/// # Errors
///
/// Returns an error if:
/// - The input is empty
/// - The file does not exist
/// - Failed to read from or write to files
pub fn process_user_data(user_input: &str) -> Result<String> {
    let path = PathBuf::from("user_data.txt");
    if !path.exists() {
        return Err(anyhow::anyhow!("File not found"));
    }

    let _content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let mut result = String::new();

    if user_input.is_empty() {
        return Err(anyhow::anyhow!("Empty input"));
    }
    result.push_str("Processed: ");
    result.push_str(user_input);

    let output_path = PathBuf::from("output.txt");
    std::fs::write(&output_path, &result)
        .with_context(|| format!("Failed to write to file: {}", output_path.display()))?;

    Ok(result)
}

/// Process a string input and return a formatted string.
///
/// # Errors
///
/// Returns an error if the input is empty.
pub fn process_input(input: &str) -> Result<String> {
    if input.is_empty() {
        return Err(anyhow::anyhow!("Empty input"));
    }
    let mut result = String::from("Processed: ");
    result.push_str(input);
    Ok(result)
}

/// Analyze a file and return its processed contents.
///
/// # Errors
///
/// Returns an error if:
/// - Failed to read the file
/// - The file contents are empty
pub fn analyze_file(path: &PathBuf) -> Result<String> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;
    process_input(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() {
        let result = process_input("test").unwrap();
        assert_eq!(result, "Processed: test");

        let empty_result = process_input("");
        assert!(empty_result.is_err());
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let path = PathBuf::from("examples/test_analysis.rs");
    let user_input = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;
    let processed = process_input(&user_input)?;
    println!("{processed}");
    Ok(())
}
