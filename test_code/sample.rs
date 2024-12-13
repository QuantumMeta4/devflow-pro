use std::fs;
use std::path::Path;

pub fn process_file(path: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    
    // Potential security issue: no path validation
    if Path::new(path).exists() {
        // Potential performance issue: unnecessary clone
        let processed = contents.clone();
        
        // Potential memory issue: large string allocation
        let mut result = String::with_capacity(processed.len() * 2);
        result.push_str(&processed);
        
        Ok(result)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found"
        ))
    }
}
