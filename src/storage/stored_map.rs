use core::marker::PhantomData;
use ethnum::u256;

use crate::{blockchain::AddressHash, math::abi::encode_pointer, AsBytes, Context};

use super::{StorageKey, StorageValue};
use alloc::rc::Rc;

pub struct StoredMap<K, V>
where
    K: Into<StorageKey>,
    V: Into<StorageValue> + Clone,
{
    context: Rc<dyn Context>,
    default: V,
    pointer: u16,
    k: PhantomData<K>,
    v: PhantomData<V>,
}

impl<K, V> StoredMap<K, V>
where
    K: Into<StorageKey> + Copy,
    V: Into<StorageValue> + From<StorageValue> + Clone,
{
    pub const fn new(context: Rc<dyn Context>, pointer: u16, default: V) -> Self {
        Self {
            context,
            default,
            pointer,
            k: PhantomData,
            v: PhantomData,
        }
    }

    pub fn set(&self, key: &K, value: V) {
        let key: StorageKey = (*key).into();
        let key_hash = encode_pointer(self.pointer, &key.0);
        let value = Into::<StorageValue>::into(value);
        self.context.store(key_hash, value);
    }

    pub fn get(&self, key: &K) -> V {
        let key: StorageKey = (*key).into();
        let key_hash = encode_pointer(self.pointer, &key.0);
        self.context
            .load(&key_hash)
            .map(|value| V::from(value))
            .unwrap_or(self.default.clone())
    }

    pub fn contains_key(&self, key: &K) -> bool {
        let key: StorageKey = (*key).into();
        let key_hash = encode_pointer(self.pointer, key.as_bytes());
        let has = self.context.exists(&key_hash);
        has
    }
}

pub type StoredAddresValueMap = StoredMap<AddressHash, u256>;

#[cfg(test)]
mod tests {
    use alloc::rc::Rc;

    use crate::{
        blockchain::AddressHash, storage::StorageValue, tests::random_address, AsBytes, TestContext,
    };

    use super::StoredMap;

    #[test]
    fn test1() {
        let context = Rc::new(TestContext::default());
        let one = [1; 32];
        let address1 = random_address();
        let sm: StoredMap<AddressHash, StorageValue> =
            StoredMap::new(context.clone(), 0, StorageValue::ZERO);
        sm.set(&address1, StorageValue::new(one));
        assert_eq!(sm.get(&address1).as_bytes(), &one);
    }
}
