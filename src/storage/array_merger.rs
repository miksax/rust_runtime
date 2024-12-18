use alloc::vec::Vec;

use crate::{math::abi::encode_pointer, storage::StorageKey};

use super::{GlobalStore, StorageValue};

#[derive(Clone, Eq, PartialEq)]
pub struct ArrayMerger {
    parent_key: Vec<u8>,
    pointer: u16,
    default_value: StorageValue,
}

impl ArrayMerger {
    pub fn new(parent_key: Vec<u8>, pointer: u16, default_value: StorageValue) -> Self {
        Self {
            parent_key,
            pointer,
            default_value,
        }
    }
    pub fn get(&mut self, key: &[u8]) -> StorageValue {
        let key = self.get_key_hash(key);
        GlobalStore::get(&key, self.default_value)
    }

    pub fn set(&mut self, key: &[u8], value: StorageValue) {
        let key = self.get_key_hash(key);
        GlobalStore::set(key, value);
    }

    pub fn contains_key(&self, key: &[u8]) -> bool {
        let key = self.get_key_hash(key);
        GlobalStore::has_key(&key)
    }

    fn get_key_hash(&self, key: &[u8]) -> StorageKey {
        let merged: Vec<u8> = self.parent_key.iter().chain(key.iter()).cloned().collect();
        encode_pointer(self.pointer, &merged)
    }
}
