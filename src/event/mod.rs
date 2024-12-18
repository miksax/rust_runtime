use crate::{blockchain::AddressHash, constant::ADDRESS_BYTE_LENGTH, WaBuffer};
use ethnum::u256;

pub trait EventTrait {
    fn buffer(&self) -> WaBuffer;
}

pub struct Event {
    buffer: WaBuffer,
}

impl Event {
    pub fn approve(
        owner: AddressHash,
        spender: AddressHash,
        value: u256,
    ) -> Result<Self, crate::error::Error> {
        let event_type = "Approve";
        let byte_size = ADDRESS_BYTE_LENGTH * 2 + 32;
        let mut buffer = WaBuffer::new(event_type.len() + 6 + byte_size, 1)?;
        let mut cursor = buffer.cursor();

        cursor.write_string_with_len(event_type)?;
        cursor.write_u32_le(&(byte_size as u32))?;
        cursor.write_address(&owner)?;
        cursor.write_address(&spender)?;
        cursor.write_u256_be(&value)?;

        Ok(Event { buffer })
    }

    pub fn burn(amount: u256) -> Result<Self, crate::error::Error> {
        let event_type = "Burn";
        let byte_size = 32;
        let mut buffer = WaBuffer::new(event_type.len() + 6 + byte_size, 1)?;
        let mut cursor = buffer.cursor();

        cursor.write_string_with_len(event_type)?;
        cursor.write_u32_le(&(byte_size as u32))?;
        cursor.write_u256_be(&amount)?;

        Ok(Event { buffer })
    }

    pub fn claim(amount: u256) -> Result<Self, crate::error::Error> {
        let event_type = "Claim";
        let byte_size = 32;
        let mut buffer = WaBuffer::new(event_type.len() + 6 + byte_size, 1)?;
        let mut cursor = buffer.cursor();

        cursor.write_string_with_len(event_type)?;
        cursor.write_u32_le(&(byte_size as u32))?;
        cursor.write_u256_be(&amount)?;

        Ok(Event { buffer })
    }

    pub fn mint(address: AddressHash, amount: u256) -> Result<Self, crate::error::Error> {
        let event_type = "Mint";
        let byte_size = 32 + ADDRESS_BYTE_LENGTH;
        let mut buffer = WaBuffer::new(event_type.len() + 6 + byte_size, 1)?;
        let mut cursor = buffer.cursor();

        cursor.write_string_with_len(event_type)?;
        cursor.write_u32_le(&(byte_size as u32))?;
        cursor.write_address(&address)?;
        cursor.write_u256_be(&amount)?;

        Ok(Event { buffer })
    }

    pub fn stake(amount: u256) -> Result<Self, crate::error::Error> {
        let event_type = "Stake";
        let byte_size = 32;
        let mut buffer = WaBuffer::new(event_type.len() + 6 + byte_size, 1)?;
        let mut cursor = buffer.cursor();

        cursor.write_string_with_len(event_type)?;
        cursor.write_u32_le(&(byte_size as u32))?;
        cursor.write_u256_be(&amount)?;

        Ok(Event { buffer })
    }

    pub fn unstake(amount: u256) -> Result<Self, crate::error::Error> {
        let event_type = "Unstake";
        let byte_size = 32;
        let mut buffer = WaBuffer::new(event_type.len() + 6 + byte_size, 1)?;
        let mut cursor = buffer.cursor();

        cursor.write_string_with_len(event_type)?;
        cursor.write_u32_le(&(byte_size as u32))?;
        cursor.write_u256_be(&amount)?;

        Ok(Event { buffer })
    }

    pub fn transfer(
        addr_from: AddressHash,
        addr_to: AddressHash,
        amount: u256,
    ) -> Result<Self, crate::error::Error> {
        let event_type = "Transfer";

        let byte_size = ADDRESS_BYTE_LENGTH * 2 + 32;
        let mut buffer = WaBuffer::new(event_type.len() + 6 + byte_size, 1)?;
        let mut cursor = buffer.cursor();

        cursor.write_string_with_len(event_type)?;

        cursor.write_u32_le(&(byte_size as u32))?;

        cursor.write_address(&addr_from)?;
        cursor.write_address(&addr_to)?;
        cursor.write_u256_be(&amount)?;

        Ok(Event { buffer })
    }
}

impl EventTrait for Event {
    fn buffer(&self) -> WaBuffer {
        self.buffer.clone()
    }
}
