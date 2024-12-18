use core::marker::PhantomData;
use ethnum::u256;

use crate::{blockchain::AddressHash, math::abi::encode_pointer};

use super::{GlobalStore, StorageKey, StorageValue};

pub struct StoredMap<K, V>
where
    K: Into<StorageKey>,
    V: Into<StorageValue>,
{
    pointer: u16,
    k: PhantomData<K>,
    v: PhantomData<V>,
}

impl<K, V> StoredMap<K, V>
where
    K: Into<StorageKey> + Copy,
    V: Into<StorageValue> + Clone,
{
    pub const fn new(pointer: u16) -> Self {
        Self {
            pointer,
            k: PhantomData,
            v: PhantomData,
        }
    }

    pub fn set(&self, key: &K, value: V) {
        let key: StorageKey = (*key).into();
        let key_hash = encode_pointer(self.pointer, &key);
        let value = Into::<StorageValue>::into(value);
        GlobalStore::set(key_hash, value);
    }

    pub fn get(&self, key: &K, default_value: V) -> StorageValue {
        let key: StorageKey = (*key).into();
        let key_hash = encode_pointer(self.pointer, &key);
        let value = GlobalStore::get(&key_hash, default_value.into());
        value
    }

    pub fn contains_key(&self, key: &K) -> bool {
        let key: StorageKey = (*key).into();
        let key_hash = encode_pointer(self.pointer, &key);
        let has = GlobalStore::has_key(&key_hash);
        has
    }
}

pub type StoredAddresValueMap = StoredMap<AddressHash, u256>;
