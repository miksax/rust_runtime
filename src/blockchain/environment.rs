use crate::WaPtr;

use super::AddressHash;
use crate::{AsBytes, ToHex};

#[derive(Clone, Copy)]
pub struct Environment {
    pub block_hash: super::BlockHash,
    pub block_number: u64,
    pub block_median_time: u64,
    pub transaction_hash: super::TransactionHash,

    pub contract_address: super::AddressHash,
    pub contract_deployer: super::AddressHash,
    pub caller: super::AddressHash,
    pub origin: super::AddressHash,
}

impl Environment {
    pub fn ptr(&self) -> WaPtr {
        WaPtr(self as *const Environment as u32)
    }
}

#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
mod display {
    use crate::utils::ToHex;
    use core::fmt::Display;

    impl Display for super::Environment {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Environment")
                .field("block_hash", &self.block_hash.to_hex())
                .field("block_number", &self.block_number)
                .field("block_median_time", &self.block_median_time)
                .field("transaction_hash", &self.transaction_hash.to_hex())
                .field("contract_address", &self.contract_address.to_hex())
                .field("contract_deployer", &self.contract_deployer.to_hex())
                .field("caller", &self.caller.to_hex())
                .field("origin", &self.origin.to_hex())
                .finish()
        }
    }
}

#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
impl Default for Environment {
    fn default() -> Self {
        Environment {
            block_hash: crate::tests::random_block(),
            block_number: crate::tests::random_u64(),
            block_median_time: crate::tests::random_u64(),
            transaction_hash: crate::tests::random_transaction(),
            contract_address: crate::tests::random_address(),
            contract_deployer: crate::tests::random_address(),
            caller: crate::tests::random_address(),
            origin: crate::tests::random_address(),
        }
    }
}
