use std::collections::HashMap;
use sha2::{Sha256, Sha512, Digest};

pub struct HashTable {
    map: HashMap<Vec<u8>, usize>
}

impl HashTable {
    pub fn insert(&mut self, key: usize, value: usize) {
        let mut hasher = Sha256::new();
        hasher.update(key.to_string());
        let result = hasher.finalize().to_vec();

        self.map.insert(result, value);
    }

    pub fn retrieve(&self, key: usize) -> Option<&usize> {
        let mut hasher = Sha256::new();
        hasher.update(key.to_string());
        let key_vec = hasher.finalize().to_vec();
        self.map.get(&key_vec)
    }
}

pub fn create_hash_table() -> HashTable {
       HashTable {
        map: HashMap::new()
       }
}