use crate::{AsBytes, FromBytes, ToHex};

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct TransactionHash([u8; crate::constant::TRANSACTION_HASH_BYTE_LENGTH]);
impl TransactionHash {
    pub const EMPTY: TransactionHash =
        TransactionHash([0; crate::constant::TRANSACTION_HASH_BYTE_LENGTH]);
}
impl AsBytes for TransactionHash {
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl FromBytes for TransactionHash {
    fn from_bytes(bytes: &[u8]) -> Self {
        Self(bytes.try_into().unwrap())
    }
}
impl ToHex for TransactionHash {}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub sender: super::AddressHash,
    pub origin: super::AddressHash,
    pub hash: TransactionHash,
}

#[derive(Clone, Debug)]
pub struct Output {
    pub index: u8,
    pub script_pub_key: [u8; 32],
    pub value: u64,
}

#[derive(Clone, Debug)]
pub struct Input {
    pub tx_id: [u8; 32],
    pub output_index: u8,
    pub script_sig: u8,
}
