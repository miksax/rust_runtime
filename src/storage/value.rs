use ethnum::u256;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StorageValue {
    inner: [u8; crate::constant::STORE_VALUE_SIZE],
}

impl StorageValue {
    pub const ZERO: StorageValue = StorageValue {
        inner: [0; crate::constant::STORE_VALUE_SIZE],
    };

    pub fn from_bytes(bytes: [u8; crate::constant::STORE_VALUE_SIZE]) -> Self {
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
        self.inner[31]
    }

    pub fn u16(&self) -> u16 {
        crate::u16_from_le(&self.inner[30..32])
    }

    pub fn u32(&self) -> u32 {
        crate::u32_from_le(&self.inner[28..32])
    }

    pub fn u64(&self) -> u64 {
        crate::u64_from_le(&self.inner[24..32])
    }

    pub fn u128(&self) -> u128 {
        crate::u128_from_le(&self.inner[16..32])
    }

    pub fn u256(&self) -> u256 {
        crate::u256_from_be(&self.inner)
    }
}

impl From<[u8; crate::constant::STORE_VALUE_SIZE]> for StorageValue {
    fn from(value: [u8; crate::constant::STORE_VALUE_SIZE]) -> Self {
        StorageValue { inner: value }
    }
}

impl From<&[u8; crate::constant::STORE_VALUE_SIZE]> for StorageValue {
    fn from(value: &[u8; crate::constant::STORE_VALUE_SIZE]) -> Self {
        StorageValue { inner: *value }
    }
}

impl From<&[u8]> for StorageValue {
    fn from(value: &[u8]) -> Self {
        let mut inner = [0u8; crate::constant::STORE_VALUE_SIZE];
        let length = value.len().min(crate::constant::STORE_VALUE_SIZE);

        inner[32 - length..32].copy_from_slice(&value[0..length]);
        StorageValue { inner }
    }
}

impl From<u8> for StorageValue {
    fn from(value: u8) -> Self {
        From::<&[u8]>::from(&value.to_be_bytes())
    }
}

impl From<u16> for StorageValue {
    fn from(value: u16) -> Self {
        From::<&[u8]>::from(&crate::u16_to_le(value))
    }
}

impl From<u32> for StorageValue {
    fn from(value: u32) -> Self {
        From::<&[u8]>::from(&crate::u32_to_le(value))
    }
}

impl From<u64> for StorageValue {
    fn from(value: u64) -> Self {
        From::<&[u8]>::from(&crate::u64_to_le(value))
    }
}

impl From<u128> for StorageValue {
    fn from(value: u128) -> Self {
        From::<&[u8]>::from(&crate::u128_to_le(value))
    }
}

impl From<u256> for StorageValue {
    fn from(value: u256) -> Self {
        Self::from(&crate::u256_to_be(value))
    }
}

impl From<bool> for StorageValue {
    fn from(value: bool) -> Self {
        From::<u8>::from(value.into())
    }
}

impl From<StorageValue> for u8 {
    fn from(val: StorageValue) -> Self {
        val.u8()
    }
}

impl From<StorageValue> for u16 {
    fn from(val: StorageValue) -> Self {
        val.u16()
    }
}

impl From<StorageValue> for u32 {
    fn from(val: StorageValue) -> Self {
        val.u32()
    }
}

impl From<StorageValue> for u64 {
    fn from(val: StorageValue) -> Self {
        val.u64()
    }
}

impl From<StorageValue> for u128 {
    fn from(val: StorageValue) -> Self {
        val.u128()
    }
}

impl From<StorageValue> for u256 {
    fn from(val: StorageValue) -> Self {
        val.u256()
    }
}

impl From<StorageValue> for bool {
    fn from(val: StorageValue) -> Self {
        val.bool()
    }
}
