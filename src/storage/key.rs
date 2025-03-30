use crate::{AsBytes, FromBytes, WaPtr};

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct StorageKey(pub [u8; crate::constant::STORE_KEY_BYTE_LENGTH]);

impl StorageKey {
    pub const fn new(bytes: [u8; crate::constant::STORE_KEY_BYTE_LENGTH]) -> Self {
        Self(bytes)
    }
    pub fn mut_ptr(&mut self) -> WaPtr {
        WaPtr(self.0.as_mut_ptr() as *mut u8 as u32)
    }

    pub fn ptr(&self) -> WaPtr {
        WaPtr(self.0.as_ptr() as *const u8 as u32)
    }
}

impl AsBytes for StorageKey {
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl FromBytes for StorageKey {
    fn from_bytes(bytes: &[u8]) -> Self {
        Self(bytes.try_into().unwrap())
    }
}
