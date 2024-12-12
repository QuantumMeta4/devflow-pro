use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Cache<K, V> {
    store: Arc<Mutex<HashMap<K, V>>>,
    max_size: usize,
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone> Cache<K, V> {
    pub fn new(max_size: usize) -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
            max_size,
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let store = self.store.lock().unwrap();
        store.get(key).cloned()
    }

    pub fn set(&self, key: K, value: V) -> Result<(), String> {
        let mut store = self.store.lock().unwrap();
        if store.len() >= self.max_size {
            return Err("Cache is full".to_string());
        }
        store.insert(key, value);
        Ok(())
    }

    pub fn clear(&self) {
        let mut store = self.store.lock().unwrap();
        store.clear();
    }

    pub fn parallel_process<F>(&self, keys: Vec<K>, mut f: F)
    where
        F: FnMut(&V) + Send + 'static,
        K: Send + 'static,
        V: Send + 'static,
    {
        let handles: Vec<_> = keys
            .into_iter()
            .filter_map(|key| {
                let store = Arc::clone(&self.store);
                let value = self.get(&key)?;
                Some(thread::spawn(move || {
                    f(&value);
                    let _guard = store.lock().unwrap();
                }))
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
