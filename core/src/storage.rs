use sha2::Sha256;
use std::collections::HashMap;

pub struct Storage {
    data: HashMap<String, String>,
    // merkle_tree: MerkleTree<String, Sha256>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            // merkle_tree: MerkleTree::new(Sha256::Hasher::new()),
        }
    }

    pub fn put(&mut self, key: String, value: String) {
        self.data.insert(key, value);
        // let hashed_value = Self::hash_value(value);
        // self.merkle_tree. add(hashed_value);
    }

    fn _hash_value(value: String) -> String {
        use sha2::Digest; // Add this line to bring Digest trait into scope
        let mut hasher = Sha256::new(); // Change this line
        hasher.update(value.as_bytes());
        format!("{:x}", hasher.finalize()) // Change this line
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.data.get(&key).cloned()
    }

    pub fn delete(&mut self, key: String) {
        // let value = self.data.get(&key).unwrap();
        // let hashed_value = Self::hash_value(value.to_string());
        // self.merkle_tree.remove(hashed_value);
        self.data.remove(&key);
    }

    // pub fn get_root(&self) -> String {
    //     self.merkle_tree.get_root().to_string()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_and_get() {
        let mut storage = Storage::new();
        storage.put("key1".to_string(), "value1".to_string());
        assert_eq!(storage.get("key1".to_string()), Some("value1".to_string()));
    }

    #[test]
    fn test_delete() {
        let mut storage = Storage::new();
        storage.put("key1".to_string(), "value1".to_string());
        storage.delete("key1".to_string());
        assert_eq!(storage.get("key1".to_string()), None);
    }

    // #[test]
    // fn test_get_root() {
    //     let mut storage = Storage::new();
    //     storage.put("key1".to_string(), "value1".to_string());
    //     let root = storage.get_root();
    //     assert!(!root.is_empty());
    // }

    // #[test]
    // fn test_root_hash_changes() {
    //     let mut storage = Storage::new();

    //     // Initial root hash
    //     let initial_root = storage.get_root();

    //     // Add first item and check root hash changes
    //     storage.put("key1".to_string(), "value1".to_string());
    //     let root_after_first_put = storage.get_root();
    //     assert_ne!(initial_root, root_after_first_put);

    //     // Add second item and check root hash changes
    //     storage.put("key2".to_string(), "value2".to_string());
    //     let root_after_second_put = storage.get_root();
    //     assert_ne!(root_after_first_put, root_after_second_put);

    //     // Remove first item and check root hash changes
    //     storage.delete("key1".to_string());
    //     let root_after_first_delete = storage.get_root();
    //     assert_ne!(root_after_second_put, root_after_first_delete);

    //     // Remove second item and check root hash changes back to initial
    //     storage.delete("key2".to_string());
    //     let root_after_second_delete = storage.get_root();
    //     assert_eq!(initial_root, root_after_second_delete);
    // }
}
