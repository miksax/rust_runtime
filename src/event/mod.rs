use crate::cursor::Cursor;
use crate::{blockchain::AddressHash, constant::ADDRESS_BYTE_LENGTH};
use ethnum::u256;

pub trait EventTrait {
    fn buffer(&self) -> &'static [u8];
    fn ptr(&self) -> u32;
}

#[derive(Clone)]
pub struct Event {
    pub buffer: &'static [u8],
}

impl Event {
    pub fn new(buffer: &'static [u8]) -> Event {
        Self { buffer }
    }

    pub fn approve(
        owner: AddressHash,
        spender: AddressHash,
        value: u256,
    ) -> Result<Self, crate::error::Error> {
        let event_type = "Approve";
        let byte_size = ADDRESS_BYTE_LENGTH * 2 + 32;
        //let mut buffer = crate::mem::WaBuffer::new(event_type.len() + 6 + byte_size, 1)?;
        let mut cursor = Cursor::new(event_type.len() + 6 + byte_size);

        cursor.write_string_with_len(event_type)?;
        cursor.write_u32(&(byte_size as u32), true)?;
        cursor.write_address(&owner)?;
        cursor.write_address(&spender)?;
        cursor.write_u256(&value, true)?;

        Ok(Event {
            buffer: cursor.into_inner(),
        })
    }

    pub fn burn(amount: u256) -> Result<Self, crate::error::Error> {
        let event_type = "Burn";
        let byte_size = 32;
        //let mut buffer = WaBuffer::new(event_type.len() + 6 + byte_size, 1)?;
        let mut cursor = Cursor::new(event_type.len() + 6 + byte_size);

        cursor.write_string_with_len(event_type)?;
        cursor.write_u32(&(byte_size as u32), true)?;
        cursor.write_u256(&amount, true)?;

        Ok(Event {
            buffer: cursor.into_inner(),
        })
    }

    pub fn claim(amount: u256) -> Result<Self, crate::error::Error> {
        let event_type = "Claim";
        let byte_size = 32;
        //let mut buffer = WaBuffer::new(event_type.len() + 6 + byte_size, 1)?;
        let mut cursor = Cursor::new(event_type.len() + 6 + byte_size);

        cursor.write_string_with_len(event_type)?;
        cursor.write_u32(&(byte_size as u32), true)?;
        cursor.write_u256(&amount, true)?;

        Ok(Event {
            buffer: cursor.into_inner(),
        })
    }

    pub fn mint(address: AddressHash, amount: u256) -> Result<Self, crate::error::Error> {
        let event_type = "Mint";
        let byte_size = 32 + ADDRESS_BYTE_LENGTH;
        //let mut buffer = WaBuffer::new(event_type.len() + 6 + byte_size, 1)?;
        let mut cursor = Cursor::new(event_type.len() + 6 + byte_size);

        cursor.write_string_with_len(event_type)?;
        cursor.write_u32(&(byte_size as u32), true)?;
        cursor.write_address(&address)?;
        cursor.write_u256(&amount, true)?;

        Ok(Event {
            buffer: cursor.into_inner(),
        })
    }

    pub fn stake(amount: u256) -> Result<Self, crate::error::Error> {
        let event_type = "Stake";
        let byte_size = 32;
        // let mut buffer = WaBuffer::new(event_type.len() + 6 + byte_size, 1)?;
        let mut cursor = Cursor::new(event_type.len() + 6 + byte_size);

        cursor.write_string_with_len(event_type)?;
        cursor.write_u32(&(byte_size as u32), true)?;
        cursor.write_u256(&amount, true)?;

        Ok(Event {
            buffer: cursor.into_inner(),
        })
    }

    pub fn unstake(amount: u256) -> Result<Self, crate::error::Error> {
        let event_type = "Unstake";
        let byte_size = 32;
        //let mut buffer = WaBuffer::new(event_type.len() + 6 + byte_size, 1)?;
        let mut cursor = Cursor::new(event_type.len() + 6 + byte_size);

        cursor.write_string_with_len(event_type)?;
        cursor.write_u32(&(byte_size as u32), true)?;
        cursor.write_u256(&amount, true)?;

        Ok(Event {
            buffer: cursor.into_inner(),
        })
    }

    pub fn transfer(
        addr_from: AddressHash,
        addr_to: AddressHash,
        amount: u256,
    ) -> Result<Self, crate::error::Error> {
        let event_type = "Transfer";

        let byte_size = ADDRESS_BYTE_LENGTH * 2 + 32;
        //let mut buffer = WaBuffer::new(event_type.len() + 6 + byte_size, 1)?;
        let mut cursor = Cursor::new(event_type.len() + 6 + byte_size);

        cursor.write_string_with_len(event_type)?;

        cursor.write_u32(&(byte_size as u32), true)?;

        cursor.write_address(&addr_from)?;
        cursor.write_address(&addr_to)?;
        cursor.write_u256(&amount, true)?;

        Ok(Event {
            buffer: cursor.into_inner(),
        })
    }
}

impl EventTrait for Event {
    fn buffer(&self) -> &'static [u8] {
        self.buffer
    }

    fn ptr(&self) -> u32 {
        self.buffer.as_ptr() as u32
    }
}
