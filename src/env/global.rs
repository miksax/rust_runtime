use alloc::format;

use crate::{
    blockchain::{
        transaction::{Input, Output},
        AddressHash, BlockHash, Environment, TransactionHash,
    },
    cursor::Cursor,
    storage::{key::StorageKey, map::Map, value::StorageValue},
    FromBytes, WaPtr,
};

use core::cell::RefCell;

#[link(wasm_import_module = "env")]
extern "C" {
    pub fn revert(data: u32, length: u32);

    pub fn exit(status: u32, data: WaPtr, length: u32);

    #[link_name = "calldata"]
    pub fn get_call_data(offset: u32, length: u32, result: WaPtr);

    #[link_name = "environment"]
    pub fn get_environment(offset: u32, length: u32, result: WaPtr);

    pub fn load(key: WaPtr, result: WaPtr);

    pub fn store(key: WaPtr, value: WaPtr);

    #[link_name = "deployFromAddress"]
    pub fn deploy_from_address(origin_address: u32, salt: u32, result_address: u32) -> u32;

    pub fn call(address: u32, call_data: u32, call_data_length: u32, resultLength: u32);

    #[link_name = "callResult"]
    pub fn call_result(offset: u32, length: u32, result: u32);

    pub fn emit(data: u32, data_length: u32);

    #[link_name = "encodeAddress"]
    pub fn encode_address(data: u32) -> u32;

    #[link_name = "validateBitcoinAddress"]
    pub fn validate_bitcoin_address(addres: u32, address_length: u32) -> u32;

    #[link_name = "verifySchnorrSignature"]
    pub fn verify_schnorr_signature(public_key: u32, signature: u32, message: u32) -> u32;

    pub fn sha256(data: u32, data_legth: u32, result: u32);

    pub fn ripemd160(data: u32, data_length: u32, result: u32);

    #[link_name = "inputsSize"]
    pub fn inputs_size() -> u32;

    pub fn inputs(buffer: WaPtr);

    #[link_name = "outputsSize"]
    pub fn outputs_size() -> u32;

    pub fn outputs(buffer: WaPtr);
}

#[link(wasm_import_module = "debug")]
extern "C" {
    pub fn log(ptr: u32, len: u32);
}

//#[cfg(target_arch = "wasm32")]
pub struct GlobalContext {
    store: RefCell<Map<StorageKey, StorageValue>>,
}

//#[cfg(target_arch = "wasm32")]
impl GlobalContext {
    pub const fn new() -> Self {
        Self {
            store: RefCell::new(Map::new()),
        }
    }
}

//#[cfg(target_arch = "wasm32")]
impl super::Context for GlobalContext {
    fn call_data(&self, size: usize) -> Cursor {
        let cursor = crate::cursor::Cursor::new(size);
        unsafe {
            get_call_data(0, size as u32, cursor.ptr());
        }
        cursor
    }

    fn environment(&self) -> Environment {
        unsafe {
            let mut cursor = Cursor::new(crate::constant::ENVIRONMENT_BYTE_LENGTH);
            get_environment(
                0,
                crate::constant::ENVIRONMENT_BYTE_LENGTH as u32,
                cursor.ptr(),
            );

            Environment {
                block_hash: BlockHash::from_bytes(
                    cursor
                        .read_bytes(crate::constant::BLOCK_HASH_BYTE_LENGTH)
                        .unwrap(),
                ),
                block_number: cursor.read_u64(true).unwrap(),
                block_median_time: cursor.read_u64(true).unwrap(),
                transaction_hash: TransactionHash::from_bytes(
                    cursor
                        .read_bytes(crate::constant::TRANSACTION_HASH_BYTE_LENGTH)
                        .unwrap(),
                ),

                contract_address: cursor.read_address().unwrap(),
                contract_deployer: cursor.read_address().unwrap(),
                caller: cursor.read_address().unwrap(),
                origin: cursor.read_address().unwrap(),
            }
        }
    }

    fn log(&self, text: &str) {
        unsafe {
            let mut cursor = Cursor::new(text.as_bytes().len() + 3);
            cursor.write_string_with_len(text).unwrap();
            log(cursor.ptr().0, cursor.write_pos() as u32);
        }
    }

    fn emit(&self, event: &dyn crate::event::EventTrait) {
        unsafe {
            let buffer = event.buffer();
            self.log(&format!(
                "Emit size: {:?} {} {}",
                buffer,
                buffer.len(),
                buffer.as_ptr() as u32
            ));
            emit(buffer.as_ptr() as u32, buffer.len() as u32);
        }
    }

    fn call(&self, address: &crate::blockchain::AddressHash, data: Cursor) -> Cursor {
        data
    }

    fn encode_address(&self, _address: &str) -> &'static [u8] {
        b"abcd"
    }

    fn deploy_from_address(
        &self,
        from_address: &AddressHash,
        salt: [u8; 32],
    ) -> Result<AddressHash, crate::error::Error> {
        Ok(*from_address)
    }

    fn validate_bitcoin_address(&self, address: &str) -> Result<bool, crate::error::Error> {
        unsafe {
            Ok(validate_bitcoin_address(
                address.as_bytes().as_ptr() as u32,
                address.as_bytes().len() as u32,
            ) != 0)
        }
    }

    fn verify_schnorr_signature(
        &self,
        address: &AddressHash,
        signature: &[u8],
        hash: &[u8],
    ) -> Result<bool, crate::error::Error> {
        unsafe {
            Ok(verify_schnorr_signature(
                address.ptr().0,
                signature.as_ptr() as u32,
                hash.as_ptr() as u32,
            ) != 0)
        }
    }

    fn load(&self, pointer: &StorageKey) -> Option<StorageValue> {
        unsafe {
            let value = StorageValue::ZERO;

            let result = self.store.borrow().get(pointer).cloned();

            if let Some(value) = result {
                Some(value)
            } else {
                load(pointer.ptr(), value.ptr());

                if value.eq(&StorageValue::ZERO) {
                    None
                } else {
                    self.store.borrow_mut().insert(*pointer, value);
                    Some(value)
                }
            }
        }
    }

    fn store(&self, pointer: StorageKey, value: StorageValue) {
        unsafe {
            if if let Some(old) = self.store.borrow().get(&pointer) {
                value.ne(old)
            } else {
                true
            } {
                self.store.borrow_mut().insert(pointer, value);
                store(pointer.ptr(), value.ptr())
            }
        }
    }

    fn exists(&self, pointer: &StorageKey) -> bool {
        if self.store.borrow().contains_key(pointer) {
            true
        } else {
            self.load(pointer).is_some()
        }
    }

    fn inputs(&self) -> alloc::vec::Vec<Input> {
        unsafe {
            let size = inputs_size();
            let buffer = Cursor::new(size as usize);
            inputs(buffer.ptr());
        }

        alloc::vec::Vec::new()
    }

    fn outputs(&self) -> alloc::vec::Vec<Output> {
        unsafe {
            let size = outputs_size();
            let buffer = Cursor::new(size as usize);
            outputs(buffer.ptr());
        }

        alloc::vec::Vec::new()
    }
}
