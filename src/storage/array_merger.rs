use alloc::vec::Vec;

use crate::{math::abi::encode_pointer, storage::StorageKey, AsBytes, Context};

use super::StorageValue;
use alloc::rc::Rc;

#[derive(Clone)]
pub struct ArrayMerger {
    context: Rc<dyn Context>,
    parent_key: Vec<u8>,
    pointer: u16,
    default_value: StorageValue,
}

impl ArrayMerger {
    pub fn new(
        context: Rc<dyn Context>,
        parent_key: Vec<u8>,
        pointer: u16,
        default_value: StorageValue,
    ) -> Self {
        Self {
            context,
            parent_key,
            pointer,
            default_value,
        }
    }
    pub fn get<T>(&mut self, key: &T) -> StorageValue
    where
        T: AsBytes,
    {
        let pointer = self.get_key_hash(key);
        self.context.load(&pointer).unwrap_or(self.default_value)
    }

    pub fn set<T>(&mut self, key: &T, value: StorageValue)
    where
        T: AsBytes,
    {
        let pointer = self.get_key_hash(key);
        self.context.store(pointer, value);
    }

    pub fn contains_key<T>(&self, key: &T) -> bool
    where
        T: AsBytes,
    {
        let key = self.get_key_hash(key);
        self.context.exists(&key)
    }

    fn get_key_hash<T>(&self, key: &T) -> StorageKey
    where
        T: AsBytes,
    {
        let merged: Vec<u8> = self
            .parent_key
            .iter()
            .chain(key.as_bytes().iter())
            .cloned()
            .collect();
        encode_pointer(self.pointer, &merged)
    }
}

impl PartialEq for ArrayMerger {
    fn eq(&self, other: &Self) -> bool {
        self.parent_key.eq(&other.parent_key) && self.pointer.eq(&other.pointer)
    }

    fn ne(&self, other: &Self) -> bool {
        self.parent_key.ne(&other.parent_key) && self.pointer.ne(&other.pointer)
    }
}

impl Eq for ArrayMerger {}

#[cfg(test)]
mod tests {
    use crate::{storage::StorageValue, AsBytes, FromBytes};

    use super::ArrayMerger;

    #[test]
    pub fn test1() {
        let context = alloc::rc::Rc::new(crate::env::TestContext::default());
        let address1 = crate::tests::random_address();
        let address2 = crate::tests::random_address();
        let mut am1 = ArrayMerger::new(context.clone(), address1.0.to_vec(), 0, StorageValue::ZERO);

        let check_value = [1; 32];

        let value = am1.get(&address2);
        assert_eq!(value.as_bytes(), StorageValue::ZERO.as_bytes());

        am1.set(&address2.as_bytes(), StorageValue::new(check_value));

        let mut am2 = ArrayMerger::new(context.clone(), address1.0.to_vec(), 0, StorageValue::ZERO);
        let value = am2.get(&address2);
        assert_eq!(value.as_bytes(), &check_value);
    }
}
