use core::marker::PhantomData;

use alloc::format;
use ethnum::u256;

use crate::{blockchain::AddressHash, math::abi::encode_pointer, to_hex};

use super::{value, GlobalStore, StorageKey, StorageValue};

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
        crate::log(
            format!(
                "Pointer set: {} {} {:?} {}",
                self.pointer,
                to_hex(&key),
                key_hash,
                value.u256()
            )
            .as_str(),
        );
        GlobalStore::set(key, value);
    }

    pub fn get(&self, key: &K, default_value: V) -> StorageValue {
        let key: StorageKey = (*key).into();
        let key_hash = encode_pointer(self.pointer, &key);
        let value = GlobalStore::get(&key, default_value.into());
        crate::log(
            format!(
                "Pointer get: {} {} {:?} {}",
                self.pointer,
                to_hex(&key),
                key_hash,
                value.u256()
            )
            .as_str(),
        );
        value
    }

    pub fn contains_key(&self, key: &K) -> bool {
        let key: StorageKey = (*key).into();
        let key_hash = encode_pointer(self.pointer, &key);
        let has = GlobalStore::has_key(&key);
        crate::log(
            format!(
                "Pointer has: {} {} {:?} {}",
                self.pointer,
                to_hex(&key),
                key_hash,
                has
            )
            .as_str(),
        );
        has
    }
}

pub type StoredAddresValueMap = StoredMap<AddressHash, u256>;
