// Sample Rust code for testing AI analysis
use std::collections::HashMap;

pub struct DataStore {
    data: HashMap<String, Vec<u8>>,
}

impl DataStore {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn store(&mut self, key: String, value: Vec<u8>) {
        // Potential optimization: Consider pre-allocating capacity
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&value);
        self.data.insert(key, buffer);
    }

    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        // Potential security issue: Using unwrap
        self.data.get(key)
    }

    pub fn unsafe_operation(&self) -> *const u8 {
        // Security issue: Using unsafe block
        unsafe {
            if let Some(value) = self.data.get("key") {
                value.as_ptr()
            } else {
                std::ptr::null()
            }
        }
    }

    pub fn process_data(&mut self, key: &str) -> String {
        // Performance issue: Inefficient string concatenation
        let mut result = String::new();
        if let Some(data) = self.data.get(key) {
            for byte in data {
                result = result + &format!("{:02x}", byte);
            }
        }
        result
    }

    pub fn clear(&mut self) {
        // Memory issue: Not optimizing memory usage
        self.data = HashMap::new();
    }
}

// A complex function with potential issues
pub fn process_data(input: Vec<String>) -> Result<HashMap<String, i32>, String> {
    let mut data = HashMap::new();

    for item in input {
        // Unsafe unwrap usage
        let value = item.parse::<i32>().unwrap();

        // Potential clone overhead
        let key = item.clone();

        // Using push_str for string concatenation
        let mut modified_key = String::new();
        modified_key.push_str(&key);
        modified_key.push_str("_processed");

        // Unsafe block
        unsafe {
            let ptr = &value as *const i32;
            println!("Memory address: {:?}", ptr);
        }

        data.insert(modified_key, value);
    }

    Ok(data)
}
