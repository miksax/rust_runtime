use crate::{storage::StorageKey, AsBytes, FromBytes, ToHex, WaPtr};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct AddressHash(pub [u8; crate::constant::ADDRESS_BYTE_LENGTH]);

impl AsBytes for AddressHash {
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl AsBytes for &AddressHash {
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl FromBytes for AddressHash {
    fn from_bytes(bytes: &[u8]) -> Self {
        Self(bytes.try_into().unwrap())
    }
}

impl ToHex for AddressHash {}
impl ToHex for &AddressHash {}

impl From<AddressHash> for StorageKey {
    fn from(val: AddressHash) -> Self {
        StorageKey::new(val.0)
    }
}

impl AddressHash {
    pub const DEAD: AddressHash = AddressHash([
        40, 74, 228, 172, 219, 50, 169, 155, 163, 235, 250, 102, 169, 29, 219, 65, 167, 183, 161,
        210, 254, 244, 21, 57, 153, 34, 205, 138, 4, 72, 92, 2,
    ]);
    pub const EMPTY: AddressHash = AddressHash([0; crate::constant::ADDRESS_BYTE_LENGTH]);
    pub const fn new(bytes: [u8; crate::constant::ADDRESS_BYTE_LENGTH]) -> Self {
        Self(bytes)
    }

    pub fn ptr(&self) -> WaPtr {
        WaPtr::from(&self.0)
    }
}
