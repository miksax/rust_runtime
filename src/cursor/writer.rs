use ethnum::u256;

use crate::{blockchain::AddressHash, constant::ADDRESS_BYTE_LENGTH, types::Selector};

impl super::Cursor {
    pub fn write_u8(&mut self, val: u8) -> Result<(), crate::error::Error> {
        if self.writer < self.inner.len() {
            self.inner[self.writer] = val;
            self.writer += 1;
            Ok(())
        } else {
            Err(crate::error::Error::BufferIsFull)
        }
    }

    pub fn write_u16_le(&mut self, val: &u16) -> Result<(), crate::error::Error> {
        if self.writer + 2 <= self.inner.len() {
            self.inner[self.writer..self.writer + 2].copy_from_slice(&val.to_le_bytes());
            self.writer += 2;
            Ok(())
        } else {
            Err(crate::error::Error::BufferIsFull)
        }
    }

    pub fn write_u32_le(&mut self, val: &u32) -> Result<(), crate::error::Error> {
        if self.writer + 4 <= self.inner.len() {
            self.inner[self.writer..self.writer + 4].copy_from_slice(&val.to_le_bytes());
            self.writer += 4;
            Ok(())
        } else {
            Err(crate::error::Error::BufferIsFull)
        }
    }

    pub fn write_u64_le(&mut self, val: &u64) -> Result<(), crate::error::Error> {
        if self.writer + 8 <= self.inner.len() {
            self.inner[self.writer..self.writer + 8].copy_from_slice(&val.to_le_bytes());
            self.writer += 8;
            Ok(())
        } else {
            Err(crate::error::Error::BufferIsFull)
        }
    }

    pub fn write_u128_le(&mut self, val: &u128) -> Result<(), crate::error::Error> {
        if self.writer + 16 <= self.inner.len() {
            self.inner[self.writer..self.writer + 16].copy_from_slice(&val.to_le_bytes());
            self.writer += 16;
            Ok(())
        } else {
            Err(crate::error::Error::BufferIsFull)
        }
    }

    pub fn write_u256_le(&mut self, val: &u256) -> Result<(), crate::error::Error> {
        if self.writer + 32 <= self.inner.len() {
            self.inner[self.writer..self.writer + 32].copy_from_slice(&val.to_le_bytes());
            self.writer += 32;
            Ok(())
        } else {
            Err(crate::error::Error::BufferIsFull)
        }
    }

    pub fn write_u256_be(&mut self, val: &u256) -> Result<(), crate::error::Error> {
        if self.writer + 32 <= self.inner.len() {
            self.inner[self.writer..self.writer + 32].copy_from_slice(&val.to_be_bytes());
            self.writer += 32;
            Ok(())
        } else {
            Err(crate::error::Error::BufferIsFull)
        }
    }

    pub fn write_bool(&mut self, val: bool) -> Result<(), crate::error::Error> {
        self.write_u8(val.into())
    }

    pub fn write_selector(&mut self, selector: &Selector) -> Result<(), crate::error::Error> {
        self.write_u32_le(selector)
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), crate::error::Error> {
        if self.writer + bytes.len() <= self.inner.len() {
            self.inner[self.writer..self.writer + bytes.len()].copy_from_slice(bytes);
            self.writer += bytes.len();
            Ok(())
        } else {
            Err(crate::error::Error::BufferIsFull)
        }
    }

    pub fn write_bytes_with_len(&mut self, bytes: &[u8]) -> Result<(), crate::error::Error> {
        if self.writer + bytes.len() + 4 <= self.inner.len() {
            self.write_u32_le(&(bytes.len() as u32))?;
            self.write_bytes(bytes)
        } else {
            Err(crate::error::Error::BufferIsFull)
        }
    }

    pub fn write_string(&mut self, string: &str) -> Result<(), crate::error::Error> {
        if self.writer + string.len() <= self.inner.len() {
            self.write_bytes(string.as_bytes())
        } else {
            Err(crate::error::Error::BufferIsFull)
        }
    }

    pub fn write_string_with_len(&mut self, string: &str) -> Result<(), crate::error::Error> {
        let bytes = string.as_bytes();
        if self.writer + bytes.len() + 2 <= self.inner.len() {
            self.write_u16_le(&(bytes.len() as u16))?;
            self.write_bytes(bytes)
        } else {
            Err(crate::error::Error::BufferIsFull)
        }
    }

    pub fn write_address(&mut self, address: &AddressHash) -> Result<(), crate::error::Error> {
        if self.writer + ADDRESS_BYTE_LENGTH <= self.inner.len() {
            self.write_bytes(&address.bytes)
        } else {
            Err(crate::error::Error::BufferIsFull)
        }
    }
}
