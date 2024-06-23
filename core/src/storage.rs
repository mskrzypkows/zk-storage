use rs_merkle::{algorithms::Sha256 as RsSha256, Hasher, MerkleTree};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub struct Storage {
    data: HashMap<String, String>,
    merkle_tree: MerkleTree<RsSha256>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            merkle_tree: MerkleTree::new(),
        }
    }

    pub fn put(&mut self, key: String, value: String) {
        self.data.insert(key, value.clone());
        let hashed_value = Self::hash_value(value);
        self.merkle_tree.insert(hashed_value);
        self.merkle_tree.commit();
    }

    fn hash_value(value: String) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(value.as_bytes());
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.data.get(&key).cloned()
    }

    /// Deletion is expensive because it requires rebuilding the entire tree.
    pub fn delete(&mut self, key: String) {
        if self.data.remove(&key).is_some() {
            let leaves: Vec<_> = self.data.values().map(|value| Self::hash_value(value.clone())).collect();
            self.merkle_tree = MerkleTree::from_leaves(&leaves);
            self.merkle_tree.commit();
        }
    }

    pub fn get_root(&self) -> Option<[u8; 32]> {
        self.merkle_tree.root()
    }
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

    #[test]
    fn test_get_root() {
        let mut storage = Storage::new();
        storage.put("key1".to_string(), "value1".to_string());
        let root = storage.get_root();
        assert!(root.is_some());
    }

    #[test]
    fn test_root_hash_changes() {
        let mut storage = Storage::new();

        // Initial root hash
        let initial_root = storage.get_root();

        // Add first item and check root hash changes
        storage.put("key1".to_string(), "value1".to_string());
        let root_after_first_put = storage.get_root();
        assert_ne!(initial_root, root_after_first_put);

        // Add second item and check root hash changes
        storage.put("key2".to_string(), "value2".to_string());
        let root_after_second_put = storage.get_root();
        assert_ne!(root_after_first_put, root_after_second_put);

        // Note: Deletion is not supported directly in this implementation.
        // You would need to rebuild the tree without the removed element.
    }

    #[test]
    fn test_root_hash_changes_on_delete() {
        let mut storage = Storage::new();

        // Add items
        storage.put("key1".to_string(), "value1".to_string());
        storage.put("key2".to_string(), "value2".to_string());

        // Root hash after adding items
        let root_after_put = storage.get_root();

        // Delete an item and check root hash changes
        storage.delete("key1".to_string());
        let root_after_delete = storage.get_root();
        assert_ne!(root_after_put, root_after_delete);
    }
}
