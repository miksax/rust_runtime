use crate::{blockchain::AddressHash, storage::map::Map, types::Selector};
use ethnum::u256;

impl super::Cursor {
    pub fn read_u32_le_unchecked(&mut self) -> u32 {
        self.reader += 4;
        u32::from_le_bytes(self.inner[self.reader - 4..self.reader].try_into().unwrap())
    }

    pub fn read_u8(&mut self) -> Result<u8, crate::error::Error> {
        if self.reader < self.inner.len() {
            let result = self.inner[self.reader];
            self.reader += 1;
            Ok(result)
        } else {
            Err(crate::error::Error::NoMoreData)
        }
    }

    pub fn read_u16_le(&mut self) -> Result<u16, crate::error::Error> {
        if self.reader + 2 <= self.inner.len() {
            let result =
                u16::from_le_bytes(self.inner[self.reader..self.reader + 2].try_into().unwrap());
            self.reader += 2;
            Ok(result)
        } else {
            Err(crate::error::Error::NoMoreData)
        }
    }

    pub fn read_u32_le(&mut self) -> Result<u32, crate::error::Error> {
        if self.reader + 4 <= self.inner.len() {
            let result =
                u32::from_le_bytes(self.inner[self.reader..self.reader + 4].try_into().unwrap());
            self.reader += 4;
            Ok(result)
        } else {
            Err(crate::error::Error::NoMoreData)
        }
    }

    pub fn read_u64_le(&mut self) -> Result<u64, crate::error::Error> {
        if self.reader + 8 <= self.inner.len() {
            let result =
                u64::from_le_bytes(self.inner[self.reader..self.reader + 8].try_into().unwrap());
            self.reader += 8;
            Ok(result)
        } else {
            Err(crate::error::Error::NoMoreData)
        }
    }

    pub fn read_u128_le(&mut self) -> Result<u128, crate::error::Error> {
        if self.reader + 16 <= self.inner.len() {
            let result = u128::from_le_bytes(
                self.inner[self.reader..self.reader + 16]
                    .try_into()
                    .unwrap(),
            );
            self.reader += 16;
            Ok(result)
        } else {
            Err(crate::error::Error::NoMoreData)
        }
    }

    pub fn read_u256_be(&mut self) -> Result<u256, crate::error::Error> {
        if self.reader + 32 <= self.inner.len() {
            let result = u256::from_be_bytes(
                self.inner[self.reader..self.reader + 32]
                    .try_into()
                    .unwrap(),
            );
            self.reader += 32;
            Ok(result)
        } else {
            Err(crate::error::Error::NoMoreData)
        }
    }

    pub fn read_u256_le(&mut self) -> Result<u256, crate::error::Error> {
        if self.reader + 32 <= self.inner.len() {
            let result = u256::from_le_bytes(
                self.inner[self.reader..self.reader + 32]
                    .try_into()
                    .unwrap(),
            );
            self.reader += 32;
            Ok(result)
        } else {
            Err(crate::error::Error::NoMoreData)
        }
    }

    pub fn read_bool(&mut self) -> Result<bool, crate::error::Error> {
        Ok((self.read_u8()?) != 0)
    }

    pub fn read_selector(&mut self) -> Result<Selector, crate::error::Error> {
        self.read_u32_le()
    }

    pub fn read_bytes(&mut self, size: usize) -> Result<&[u8], crate::error::Error> {
        if self.reader + size <= self.inner.len() {
            let result = &self.inner[self.reader..self.reader + size];
            self.reader += size;
            Ok(result)
        } else {
            Err(crate::error::Error::NoMoreData)
        }
    }

    pub fn read_address(&mut self) -> Result<AddressHash, crate::error::Error> {
        Ok(AddressHash::new(
            self.read_bytes(crate::constant::ADDRESS_BYTE_LENGTH)?,
        ))
    }

    pub fn read_address_value_map(
        &mut self,
    ) -> Result<Map<AddressHash, u256>, crate::error::Error> {
        let len = self.read_u16_le()?;
        let mut result = Map::new();

        for _ in 0..len {
            result.insert(self.read_address()?, self.read_u256_be()?);
        }
        Ok(result)
    }
}
