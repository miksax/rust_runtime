use crate::{AsBytes, FromBytes, ToHex};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BlockHash([u8; crate::constant::BLOCK_HASH_BYTE_LENGTH]);
impl BlockHash {
    pub const EMPTY: BlockHash = BlockHash([0; crate::constant::BLOCK_HASH_BYTE_LENGTH]);
}

impl AsBytes for BlockHash {
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl FromBytes for BlockHash {
    fn from_bytes(bytes: &[u8]) -> Self {
        Self(bytes.try_into().unwrap())
    }
}

impl ToHex for BlockHash {}
