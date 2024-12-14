// Sample Rust code for testing AI analysis
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct DataStore {
    data: HashMap<String, Vec<u8>>,
}

impl DataStore {
    /// Creates a new `DataStore` instance.
    #[must_use = "This function returns a new DataStore instance that should be used"]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: String, value: Vec<u8>) {
        self.data.insert(key, value);
    }

    /// Gets the value associated with the given key.
    #[must_use = "This function returns an Option that should be used"]
    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        self.data.get(key)
    }

    /// Converts bytes to a hex string.
    #[must_use = "This function returns a String that should be used"]
    pub fn bytes_to_hex(bytes: &[u8]) -> String {
        bytes.iter().fold(String::new(), |mut acc, b| {
            acc.push_str(&format!("{b:02x}"));
            acc
        })
    }

    /// Process data associated with the given key.
    #[must_use]
    pub fn process_data(&self, key: &str) -> String {
        Self::bytes_to_hex(self.get(key).unwrap_or(&vec![]))
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

/// Process input data and return a `HashMap` with processed values.
///
/// # Errors
///
/// Returns an error if the input data cannot be processed.
pub fn process_data(input: &[String]) -> Result<HashMap<String, i32>, String> {
    let mut result = HashMap::new();
    for item in input {
        let value = item.parse::<i32>().map_err(|e| e.to_string())?;
        result.insert(item.clone(), value);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_store() {
        let mut store = DataStore::new();
        store.insert("key1".to_string(), vec![1, 2, 3]);
        assert_eq!(store.get("key1"), Some(&vec![1, 2, 3]));
        assert_eq!(store.process_data("key1"), "010203");
        store.clear();
        assert_eq!(store.get("key1"), None);
    }

    #[test]
    fn test_multiple_operations() {
        let mut store = DataStore::new();
        store.insert("key1".to_string(), vec![1, 2, 3]);
        store.insert("key2".to_string(), vec![4, 5, 6]);
        assert_eq!(store.get("key1"), Some(&vec![1, 2, 3]));
        assert_eq!(store.get("key2"), Some(&vec![4, 5, 6]));
        assert_eq!(store.process_data("key1"), "010203");
        assert_eq!(store.process_data("key2"), "040506");
        store.clear();
        assert_eq!(store.get("key1"), None);
        assert_eq!(store.get("key2"), None);
    }
}
