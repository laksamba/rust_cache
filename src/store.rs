use std::collections::HashMap;

pub struct Cache {
    store: HashMap<String, String>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            store: HashMap::new(),
        }
    }

    // set key to a given value

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    // get a reference to a value associated with a key
    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    // delete a key  from the cache , returning the previous value if it existed
    pub fn del(&mut self, key: &str) -> Option<String> {
        self.store.remove(key)
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_cache_operations() {
        let mut cache = Cache::new();

        // test Get on missing key
        assert_eq!(cache.get("nonexistent"), None);

        // Test Set and Get
        cache.set("foo".to_string(), "bar".to_string());
        assert_eq!(cache.get("foo"), Some(&"bar".to_string()));

        // Test del
        let removed = cache.del("foo");
        assert_eq!(removed, Some("bar".to_string()));
        assert_eq!(cache.get("foo"), None);
    }
}
