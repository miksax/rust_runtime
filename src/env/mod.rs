use crate::{
    mem::WaBuffer,
    storage::{StorageKey, StorageValue},
};
use alloc::vec::Vec;
#[allow(unused_imports)]
use core::str::FromStr;
pub mod global;

#[cfg(not(target_arch = "wasm32"))]
mod test;

#[cfg(not(target_arch = "wasm32"))]
pub use test::{Network, TestContext};

#[cfg(target_arch = "wasm32")]
pub fn sha256(bytes: &[u8]) -> &'static [u8] {
    unsafe { WaBuffer::from_raw(global::sha256(WaBuffer::from_bytes(bytes).unwrap().ptr())).data() }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn sha256(bytes: &[u8]) -> &'static [u8] {
    let sha = alloc::boxed::Box::new(sha2_const::Sha256::new().update(bytes).finalize());
    alloc::boxed::Box::leak(sha)
}

#[cfg(target_arch = "wasm32")]
pub fn ripemd160(bytes: &[u8]) -> &'static [u8] {
    unsafe {
        WaBuffer::from_raw(global::ripemd160(
            WaBuffer::from_bytes(bytes).unwrap().ptr(),
        ))
        .data()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn ripemd160(data: &[u8]) -> &'static [u8] {
    use ripemd::Digest;

    let mut hasher = ripemd::Ripemd160::new();
    hasher.update(data);
    let hash = hasher.finalize();

    alloc::boxed::Box::leak(alloc::boxed::Box::new(hash.to_vec()))
}

pub trait Context {
    fn log(&self, text: &str);
    fn emit(&mut self, event: &dyn crate::event::EventTrait);
    fn call(&self, buffer: WaBuffer) -> WaBuffer;

    fn deploy(&self, buffer: WaBuffer) -> WaBuffer;
    fn deploy_from_address(&self, buffer: WaBuffer) -> WaBuffer;

    fn load(&mut self, pointer: &StorageKey) -> Option<StorageValue>;
    fn store(&mut self, pointer: StorageKey, value: StorageValue);
    fn exists(&mut self, pointer: &StorageKey) -> bool;
    fn next_pointer_greater_than(&self, pointer: StorageKey) -> StorageKey;

    fn encode_address(&self, address: &str) -> &'static [u8];
    fn validate_bitcoin_address(&self, address: &str) -> bool;
    fn verify_schnorr_signature(&self, data: &[u8]) -> bool;
    fn sha256(&self, data: &[u8]) -> &'static [u8] {
        sha256(data)
    }
    fn sha256_double(&self, data: &[u8]) -> &'static [u8] {
        self.sha256(self.sha256(data))
    }
    fn ripemd160(&self, data: &[u8]) -> &'static [u8] {
        ripemd160(data)
    }

    fn inputs(&self) -> Vec<crate::blockchain::transaction::Input>;
    fn outputs(&self) -> Vec<crate::blockchain::transaction::Output>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sha_256() {
        let text = "Hello world";
        assert_eq!(
            crate::utils::to_hex(super::sha256(text.as_bytes())),
            alloc::string::String::from(
                "0x64ec88ca00b268e5ba1a35678a1b5316d212f4f366b2477232534a8aeca37f3c"
            )
        );
    }

    #[test]
    fn test_ripemd160() {
        let text = "Hello world";
        assert_eq!(
            crate::utils::to_hex(super::ripemd160(text.as_bytes())),
            alloc::string::String::from("0xdbea7bd24eef40a2e79387542e36dd408b77b21a")
        );
    }
}
