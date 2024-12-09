use core::default;

use ethnum::u256;

pub type StoreKey = [u8; crate::constant::STORE_KEY_SIZE];

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StorageValue {
    inner: [u8; crate::constant::STORE_VALUE_SIZE],
}

impl StorageValue {
    pub fn from_le_bytes(bytes: [u8; crate::constant::STORE_VALUE_SIZE]) -> Self {
        Self { inner: bytes }
    }
    pub fn value(&self) -> [u8; crate::constant::STORE_VALUE_SIZE] {
        self.inner
    }

    pub fn bytes(&self) -> &[u8] {
        &self.inner
    }

    pub fn bool(&self) -> bool {
        self.inner.iter().any(|v| 0.le(v))
    }

    pub fn zero(&self) -> bool {
        self.inner.iter().all(|v| 0.eq(v))
    }

    pub fn u8(&self) -> u8 {
        self.inner[0]
    }

    pub fn u16(&self) -> u16 {
        u16::from_le_bytes([self.inner[0], self.inner[1]])
    }

    pub fn u32(&self) -> u32 {
        u32::from_be_bytes(self.inner[0..4].try_into().unwrap())
    }

    pub fn u64(&self) -> u64 {
        u64::from_be_bytes(self.inner[0..8].try_into().unwrap())
    }

    pub fn u128(&self) -> u128 {
        u128::from_be_bytes(self.inner[0..16].try_into().unwrap())
    }

    pub fn u256(&self) -> u256 {
        u256::from_be_bytes(self.inner)
    }
}

impl From<[u8; crate::constant::STORE_VALUE_SIZE]> for StoreValue {
    fn from(value: [u8; crate::constant::STORE_VALUE_SIZE]) -> Self {
        StoreValue { inner: value }
    }
}

impl From<&[u8; crate::constant::STORE_VALUE_SIZE]> for StoreValue {
    fn from(value: &[u8; crate::constant::STORE_VALUE_SIZE]) -> Self {
        StoreValue {
            inner: value.clone(),
        }
    }
}

impl From<&[u8]> for StoreValue {
    fn from(value: &[u8]) -> Self {
        let mut inner = [0u8; crate::constant::STORE_VALUE_SIZE];
        for i in 0..value.len().max(crate::constant::STORE_VALUE_SIZE) {
            inner[i] = value[i];
        }
        StoreValue { inner }
    }
}

static mut GLOBAL_STORE: Map<StoreKey, StoreValue> = Map::new();

pub struct GlobalStore {}

impl GlobalStore {
    pub fn get(key: &[u8; 32], default_value: StoreValue) -> StoreValue {
        Self::ensure_key(key, default_value);

        if Self::has_key(key) {
            unsafe {
                if let Some(value) = GLOBAL_STORE.get(key) {
                    value.clone()
                } else {
                    default_value
                }
            }
        } else {
            default_value
        }
    }

    pub fn set(key: &StoreKey, value: StoreValue) {
        if let Ok(true) = crate::env::pointer_store(key, &value) {
            unsafe {
                GLOBAL_STORE.insert(key.clone(), value);
            }
        } else {
            crate::log("Storage has failed");
        }
    }

    fn has_key(pointer_hash: &StoreKey) -> bool {
        unsafe {
            if GLOBAL_STORE.contains_key(pointer_hash) {
                true;
            };

            if let Ok(result) = crate::env::pointer_load(pointer_hash) {
                GLOBAL_STORE.push(pointer_hash.clone(), result);
                return !result.zero();
            }
        }

        return false;
    }

    fn ensure_key(pointer_hash: &StoreKey, default_value: StoreValue) {
        if Self::has_key(pointer_hash) {
            return;
        }

        if default_value.zero() {
            return;
        }

        Self::set(pointer_hash, default_value);
    }
}
