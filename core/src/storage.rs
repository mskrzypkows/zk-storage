use rs_merkle::{algorithms::Sha256 as RsSha256, MerkleTree};
use sha2::{Digest, Sha256};

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytes for u32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

pub struct Storage<T> {
    data: Vec<T>,
    merkle_tree: MerkleTree<RsSha256>,
}

impl<T: Clone + std::cmp::PartialEq + ToBytes> Storage<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            merkle_tree: MerkleTree::new(),
        }
    }

    pub fn new_from_vec(data: Vec<T>) -> Self {
        let leaves: Vec<_> = data.iter().map(Self::hash_value).collect();
        let mut merkle_tree = MerkleTree::from_leaves(&leaves);
        merkle_tree.commit();
        Self { data, merkle_tree }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.data.contains(value)
    }

    pub fn put(&mut self, value: T) {
        self.data.push(value.clone());
        let hashed_value = Self::hash_value(&value);
        self.merkle_tree.insert(hashed_value);
        self.merkle_tree.commit();
    }

    fn hash_value(value: &T) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&value.to_bytes());
        hasher.finalize().into()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn delete(&mut self, index: usize) {
        if index < self.data.len() {
            self.data.remove(index);
            let leaves: Vec<_> = self.data.iter().map(Self::hash_value).collect();
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
        storage.put("value1".to_string());
        assert_eq!(storage.get(0), Some(&"value1".to_string()));
    }

    #[test]
    fn test_delete() {
        let mut storage = Storage::new();
        storage.put("value1".to_string());
        storage.delete(0);
        assert_eq!(storage.get(0), None);
    }

    #[test]
    fn test_get_root() {
        let mut storage = Storage::new();
        storage.put("value1".to_string());
        let root = storage.get_root();
        assert!(root.is_some());
    }

    #[test]
    fn test_root_hash_changes() {
        let mut storage = Storage::new();

        // Initial root hash
        let initial_root = storage.get_root();

        // Add first item and check root hash changes
        storage.put("value1".to_string());
        let root_after_first_put = storage.get_root();
        assert_ne!(initial_root, root_after_first_put);

        // Add second item and check root hash changes
        storage.put("value2".to_string());
        let root_after_second_put = storage.get_root();
        assert_ne!(root_after_first_put, root_after_second_put);

        // Note: Deletion is not supported directly in this implementation.
        // You would need to rebuild the tree without the removed element.
    }

    #[test]
    fn test_root_hash_changes_on_delete() {
        let mut storage = Storage::new();

        // Add items
        storage.put("value1".to_string());
        storage.put("value2".to_string());

        // Root hash after adding items
        let root_after_put = storage.get_root();

        // Delete an item and check root hash changes
        storage.delete(0);
        let root_after_delete = storage.get_root();
        assert_ne!(root_after_put, root_after_delete);
    }
}
