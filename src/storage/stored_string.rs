use core::cell::RefCell;

use crate::{constant::STORE_VALUE_BYTE_LENGTH, storage::StorageValue, AsBytes, Context};
use alloc::rc::Rc;
use alloc::string::String;

use super::stored::StoredTrait;

pub struct StoredString {
    context: Rc<RefCell<dyn Context>>,
    pointer: u16,
    default_value: &'static str,
    value: Option<alloc::string::String>,
}

impl StoredString {
    pub const fn new_const(
        context: Rc<RefCell<dyn Context>>,
        pointer: u16,
        default_value: &'static str,
    ) -> Self {
        Self {
            context,
            pointer,
            default_value,
            value: None,
        }
    }

    pub fn new(
        context: Rc<RefCell<dyn Context>>,
        pointer: u16,
        default_value: &'static str,
    ) -> Self {
        Self {
            context,
            pointer,
            default_value,
            value: None,
        }
    }

    fn save<'a>(&mut self, value: String) -> String {
        let bytes = value.as_bytes();
        let mut remaining = bytes.len();
        let mut offset = [0u8; crate::constant::STORE_VALUE_BYTE_LENGTH];
        assert!(remaining < 2048);
        let context = self.context.borrow();

        let mut data = [0u8; crate::constant::STORE_VALUE_BYTE_LENGTH];
        let mut length = remaining.min(STORE_VALUE_BYTE_LENGTH - 4);
        data[0..4].copy_from_slice(&(remaining as u32).to_le_bytes());
        data[4..4 + remaining.min(28)].copy_from_slice(&bytes[0..length]);
        let key = crate::math::abi::encode_pointer(self.pointer, &offset);
        remaining -= length;
        context.store(key, data.into());

        while remaining > 0 {
            length = remaining.min(crate::constant::STORE_VALUE_BYTE_LENGTH);
            let start = value.len() - remaining;
            data[0..length].copy_from_slice(&bytes[start..start + length]);
            if length < crate::constant::STORE_VALUE_BYTE_LENGTH {
                data[length..crate::constant::STORE_VALUE_BYTE_LENGTH].copy_from_slice(
                    &[0u8; crate::constant::STORE_VALUE_BYTE_LENGTH]
                        [0..crate::constant::STORE_VALUE_BYTE_LENGTH - length],
                );
            }
            offset[crate::constant::STORE_KEY_BYTE_LENGTH - 1] += 1;
            remaining -= length;

            let key = crate::math::abi::encode_pointer(self.pointer, &offset);
            context.store(key, data.into());
        }
        self.value = Some(value.clone());
        value
    }

    fn load<'a>(&mut self) -> String {
        let mut offset = [0u8; crate::constant::STORE_VALUE_BYTE_LENGTH];
        let context = self.context.borrow_mut();

        let header = context
            .load(&crate::math::abi::encode_pointer(self.pointer, &offset))
            .unwrap_or(StorageValue::ZERO);
        let bytes = header.as_bytes();
        let len = u32::from_le_bytes(bytes[0..4].try_into().unwrap()) as usize;
        let mut length = len.min(crate::constant::STORE_VALUE_BYTE_LENGTH);
        let mut value: alloc::vec::Vec<u8> = alloc::vec::Vec::with_capacity(len);
        let mut remaining = len - length;
        for &byte in bytes.iter().skip(4).take(length) {
            value.push(byte);
        }

        while remaining > 0 {
            offset[crate::constant::STORE_VALUE_BYTE_LENGTH - 1] += 1;
            let key = crate::math::abi::encode_pointer(self.pointer, &offset);
            let tmp = context.load(&key).unwrap_or(StorageValue::ZERO);
            let bytes = tmp.as_bytes();
            length = remaining.min(crate::constant::STORE_VALUE_BYTE_LENGTH);
            for &byte in bytes.iter().take(length) {
                value.push(byte);
            }
            remaining -= length;
        }
        if let Ok(value) = String::from_utf8(value) {
            self.value = Some(value.clone());
            value
            // Unexpected state
        } else {
            self.default_value.into()
        }
    }
}

impl StoredTrait<String, &'static str> for StoredString {
    fn set(&mut self, value: String) -> String {
        self.save(value)
    }

    fn value(&mut self) -> String {
        if let Some(value) = &self.value {
            value.clone()
        } else {
            self.load()
        }
    }

    fn set_no_commit(&mut self, value: String) -> String {
        self.value = Some(value.clone());
        value
    }

    fn commit(&mut self) {
        if let Some(value) = &self.value {
            self.save(value.clone());
        }
    }

    fn refresh(&mut self) -> String {
        self.load()
    }
}
