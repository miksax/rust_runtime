use crate::{
    blockchain::{AddressHash, Environment},
    cursor::Cursor,
    storage::{StorageKey, StorageValue},
};
use alloc::{collections::binary_heap::Iter, vec::Vec};
#[allow(unused_imports)]
use core::str::FromStr;
pub mod global;

#[cfg(not(target_arch = "wasm32"))]
mod test;

#[cfg(not(target_arch = "wasm32"))]
pub use test::{TestContext, TestRouter};

#[cfg(target_arch = "wasm32")]
pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    use crate::math::bytes;

    unsafe {
        //WaBuffer::from_raw(global::sha256(WaBuffer::from_bytes(bytes).unwrap().ptr())).data()
        let len = bytes.len() as u32;
        let result = [0u8; 32];
        global::sha256(
            bytes.as_ptr() as *const u8 as u32,
            (&len) as *const _ as *const u8 as u32,
            result.as_ptr() as *const u8 as u32,
        );
        result
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    let sha = sha2_const::Sha256::new().update(bytes).finalize();
    sha
}

#[cfg(target_arch = "wasm32")]
pub fn ripemd160(bytes: &[u8]) -> [u8; 20] {
    use crate::{math::bytes, WaPtr};

    let len = bytes.len() as u32;
    let result = [0u8; 20];

    unsafe {
        global::ripemd160(
            WaPtr::from(bytes).0,
            WaPtr::from(&len).0,
            WaPtr::from(&result[0..20]).0,
        );
        result
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn ripemd160(data: &[u8]) -> [u8; 20] {
    use ripemd::Digest;

    let mut hasher = ripemd::Ripemd160::new();
    hasher.update(data);
    let hash = hasher.finalize();

    hash[0..20].try_into().unwrap()
}

pub trait Context {
    fn call_data(&self, size: usize) -> Cursor;
    fn environment(&self) -> Environment;
    fn log(&self, text: &str);
    fn emit(&self, event: &dyn crate::event::EventTrait);
    fn call(&self, address: &crate::blockchain::AddressHash, data: Cursor) -> Cursor;

    fn deploy_from_address(
        &self,
        from_address: &crate::blockchain::AddressHash,
        salt: [u8; 32],
    ) -> Result<crate::blockchain::AddressHash, crate::error::Error>;

    fn load(&self, pointer: &StorageKey) -> Option<StorageValue>;
    fn store(&self, pointer: StorageKey, value: StorageValue);
    fn exists(&self, pointer: &StorageKey) -> bool;

    fn encode_address(&self, address: &str) -> &'static [u8];
    fn validate_bitcoin_address(&self, address: &str) -> Result<bool, crate::error::Error>;
    fn verify_schnorr_signature(
        &self,
        address: &AddressHash,
        signature: &[u8],
        hash: &[u8],
    ) -> Result<bool, crate::error::Error>;
    fn sha256(&self, data: &[u8]) -> [u8; 32] {
        sha256(data)
    }
    fn sha256_double(&self, data: &[u8]) -> [u8; 32] {
        self.sha256(&self.sha256(data))
    }
    fn ripemd160(&self, data: &[u8]) -> [u8; 20] {
        ripemd160(data)
    }

    fn inputs(&self) -> Vec<crate::blockchain::transaction::Input>;
    //fn iter_inputs(&mut self) -> impl Iterator<Item = &crate::blockchain::transaction::Input>;
    fn outputs(&self) -> Vec<crate::blockchain::transaction::Output>;
    //fn iter_outputs(&mut self) -> impl Iterator<Item = &crate::blockchain::transaction::Output>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sha_256() {
        let text = "Hello world";
        assert_eq!(
            crate::utils::to_hex(&super::sha256(text.as_bytes())),
            alloc::string::String::from(
                "0x64ec88ca00b268e5ba1a35678a1b5316d212f4f366b2477232534a8aeca37f3c"
            )
        );
    }

    #[test]
    fn test_ripemd160() {
        let text = "Hello world";
        assert_eq!(
            crate::utils::to_hex(&super::ripemd160(text.as_bytes())),
            alloc::string::String::from("0xdbea7bd24eef40a2e79387542e36dd408b77b21a")
        );
    }
}
