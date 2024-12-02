use crate::WaBuffer;

pub mod reader;
pub mod writer;

pub struct Cursor {
    inner: &'static mut [u8],
    reader: usize,
    writer: usize,
}

impl Cursor {
    pub fn from_slice(inner: &'static mut [u8]) -> Cursor {
        Cursor {
            reader: 0,
            writer: 0,
            inner,
        }
    }

    pub fn into_inner(self) -> &'static [u8] {
        self.inner
    }

    pub fn read_pos(&self) -> usize {
        self.reader
    }

    pub fn write_pos(&self) -> usize {
        self.writer
    }

    pub fn reset(&mut self) {
        self.reader = 0;
        self.writer = 0;
    }

    pub fn get_buffer(&self) -> WaBuffer {
        WaBuffer::from_bytes(self.inner)
    }
}

#[cfg(test)]
mod tests {
    use ethnum::u256;

    #[test]
    fn test_reader() {
        let mem = alloc::boxed::Box::new([0; 256]);
        let mut cursor = super::Cursor::from_slice(alloc::boxed::Box::leak(mem));

        cursor.write_u8(1).unwrap();
        cursor.write_u16_le(&2).unwrap();
        cursor.write_u32_le(&3).unwrap();
        cursor.write_u64_le(&4).unwrap();
        cursor.write_u128_le(&5).unwrap();
        cursor.write_u256_be(&u256::new(6)).unwrap();

        assert_eq!(cursor.read_u8().unwrap(), 1);
        assert_eq!(cursor.read_u16_le().unwrap(), 2);
        assert_eq!(cursor.read_u32_le().unwrap(), 3);
        assert_eq!(cursor.read_u64_le().unwrap(), 4);
        assert_eq!(cursor.read_u128_le().unwrap(), 5);
        assert_eq!(cursor.read_u256_be().unwrap(), u256::new(6));
    }
}
