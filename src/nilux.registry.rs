
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct NiluxRegistry {
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl NiluxRegistry {
    pub fn new() -> Self {
        NiluxRegistry {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn set(&self, key: &str, value: &str) -> Result<(), String> {
        let mut data = self.data.write().map_err(|e| e.to_string())?;
        data.insert(key.to_string(), value.to_string());
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, String> {
        let data = self.data.read().map_err(|e| e.to_string())?;
        Ok(data.get(key).cloned())
    }

    pub fn delete(&self, key: &str) -> Result<(), String> {
        let mut data = self.data.write().map_err(|e| e.to_string())?;
        data.remove(key);
        Ok(())
    }

    pub fn list_keys(&self) -> Result<Vec<String>, String> {
        let data = self.data.read().map_err(|e| e.to_string())?;
        Ok(data.keys().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_operations() {
        let registry = NiluxRegistry::new();

        // Test set and get
        registry.set("test_key", "test_value").unwrap();
        assert_eq!(registry.get("test_key").unwrap(), Some("test_value".to_string()));

        // Test update
        registry.set("test_key", "updated_value").unwrap();
        assert_eq!(registry.get("test_key").unwrap(), Some("updated_value".to_string()));

        // Test delete
        registry.delete("test_key").unwrap();
        assert_eq!(registry.get("test_key").unwrap(), None);

        // Test list_keys
        registry.set("key1", "value1").unwrap();
        registry.set("key2", "value2").unwrap();
        let keys = registry.list_keys().unwrap();
        assert!(keys.contains(&"key1".to_string()));
        assert!(keys.contains(&"key2".to_string()));
    }
}
