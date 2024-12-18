use super::AddressHash;

pub struct Environment {
    pub sender: super::AddressHash,
    pub origin: super::AddressHash,
    pub transaction_hash: super::TransactionHash,
    pub block_hash: super::BlockHash,
    pub deployer: super::AddressHash,
    pub address: super::AddressHash,
    pub timestamp: u64,
    pub safe_rnd: u64,
}

impl Environment {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        sender: AddressHash,
        origin: AddressHash,
        transaction_hash: super::TransactionHash,
        block_hash: super::BlockHash,
        deployer: AddressHash,
        address: AddressHash,
        timestamp: u64,
        safe_rnd: u64,
    ) -> Self {
        Self {
            sender,
            origin,
            transaction_hash,
            block_hash,
            deployer,
            address,
            timestamp,
            safe_rnd,
        }
    }
}

#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
mod display {
    use crate::utils::{to_hex, ToHex};
    use core::fmt::Display;

    impl Display for super::Environment {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            use alloc::string::ToString;

            f.debug_struct("Environment")
                .field("sender", &self.sender.to_hex())
                .field("origin", &self.origin.to_hex())
                .field("transaction", &to_hex(&self.transaction_hash.bytes))
                .field("block_hash", &to_hex(&self.block_hash.bytes))
                .field("deployer", &self.deployer.to_hex())
                .field("address", &self.address.to_hex())
                .field("timestamp", &self.timestamp.to_string())
                .field("safe_rnd", &self.safe_rnd.to_string())
                .finish()
        }
    }
}

impl Environment {}
