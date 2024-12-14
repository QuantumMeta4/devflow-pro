// Sample Rust code for testing AI analysis
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct DataStore {
    data: HashMap<String, Vec<u8>>,
}

impl DataStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn store(&mut self, key: String, value: Vec<u8>) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        self.data.get(key)
    }

    pub fn process_data(&mut self, key: &str) -> String {
        let mut result = String::new();
        if let Some(data) = self.data.get(key) {
            for byte in data {
                result = result + &format!("{:02x}", byte);
            }
        }
        result
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

// A complex function with potential issues
pub fn process_data(input: Vec<String>) -> Result<HashMap<String, i32>, String> {
    let mut data = HashMap::new();

    for item in input {
        let value = match item.parse::<i32>() {
            Ok(value) => value,
            Err(_) => return Err("Failed to parse value".to_string()),
        };

        let key = item.clone();

        let mut modified_key = String::new();
        modified_key.push_str(&key);
        modified_key.push_str("_processed");

        data.insert(modified_key, value);
    }

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_store() {
        let mut store = DataStore::new();
        store.store("key1".to_string(), vec![1, 2, 3]);
        assert_eq!(store.get("key1"), Some(&vec![1, 2, 3]));
        assert_eq!(store.process_data("key1"), "010203");
        store.clear();
        assert_eq!(store.get("key1"), None);
    }

    #[test]
    fn test_multiple_operations() {
        let mut store = DataStore::new();
        store.store("key1".to_string(), vec![1, 2, 3]);
        store.store("key2".to_string(), vec![4, 5, 6]);
        assert_eq!(store.get("key1"), Some(&vec![1, 2, 3]));
        assert_eq!(store.get("key2"), Some(&vec![4, 5, 6]));
        assert_eq!(store.process_data("key1"), "010203");
        assert_eq!(store.process_data("key2"), "040506");
        store.clear();
        assert_eq!(store.get("key1"), None);
        assert_eq!(store.get("key2"), None);
    }
}
