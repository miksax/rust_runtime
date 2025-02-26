use core::num;

const BASE64_TABLE: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub fn to_hex(bytes: &[u8]) -> alloc::string::String {
    let mut string = alloc::string::String::with_capacity(bytes.len() * 2 + 3);
    string.push_str("0x");

    for byte in bytes.iter() {
        string.push(BASE64_TABLE[(*byte >> 4) as usize]);
        string.push(BASE64_TABLE[(*byte & 0xf) as usize]);
    }
    string
}
pub trait ToHex {
    fn get_bytes(&self) -> &[u8];
    fn to_hex(&self) -> alloc::string::String {
        to_hex(self.get_bytes())
    }
}

impl ToHex for &[u8] {
    fn get_bytes(&self) -> &[u8] {
        self
    }
}

pub fn u16_from_le(bytes: &[u8]) -> u16 {
    (bytes[0] as u16) + ((bytes[1] as u16) << 8)
}

pub fn u16_to_le(number: u16) -> [u8; 2] {
    [(number & 0xff) as u8, ((number >> 8) & 0xff) as u8]
}

pub fn u32_from_le(bytes: &[u8]) -> u32 {
    (bytes[0] as u32)
        + ((bytes[1] as u32) << 8)
        + ((bytes[2] as u32) << 16)
        + ((bytes[3] as u32) << 24)
}

pub fn u32_to_le(number: u32) -> [u8; 4] {
    [
        (number & 0xff) as u8,
        ((number >> 8) & 0xff) as u8,
        ((number >> 16) & 0xff) as u8,
        ((number >> 24) & 0xff) as u8,
    ]
}

pub fn u64_from_le(bytes: &[u8]) -> u64 {
    (bytes[0] as u64)
        + ((bytes[1] as u64) << 8)
        + ((bytes[2] as u64) << 16)
        + ((bytes[3] as u64) << 24)
        + ((bytes[4] as u64) << 32)
        + ((bytes[5] as u64) << 40)
        + ((bytes[6] as u64) << 48)
        + ((bytes[7] as u64) << 56)
}

pub fn u64_to_le(number: u64) -> [u8; 8] {
    [
        (number & 0xff) as u8,
        ((number >> 8) & 0xff) as u8,
        ((number >> 16) & 0xff) as u8,
        ((number >> 24) & 0xff) as u8,
        ((number >> 32) & 0xff) as u8,
        ((number >> 40) & 0xff) as u8,
        ((number >> 48) & 0xff) as u8,
        ((number >> 56) & 0xff) as u8,
    ]
}

pub fn u128_from_le(bytes: &[u8]) -> u128 {
    (bytes[0] as u128)
        + ((bytes[1] as u128) << 8)
        + ((bytes[2] as u128) << 16)
        + ((bytes[3] as u128) << 24)
        + ((bytes[4] as u128) << 32)
        + ((bytes[5] as u128) << 40)
        + ((bytes[6] as u128) << 48)
        + ((bytes[7] as u128) << 56)
        + ((bytes[8] as u128) << 64)
        + ((bytes[9] as u128) << 72)
        + ((bytes[10] as u128) << 80)
        + ((bytes[11] as u128) << 88)
        + ((bytes[12] as u128) << 96)
        + ((bytes[13] as u128) << 104)
        + ((bytes[14] as u128) << 112)
        + ((bytes[15] as u128) << 120)
}

pub fn u128_to_le(number: u128) -> [u8; 16] {
    [
        (number & 0xff) as u8,
        ((number >> 8) & 0xff) as u8,
        ((number >> 16) & 0xff) as u8,
        ((number >> 24) & 0xff) as u8,
        ((number >> 32) & 0xff) as u8,
        ((number >> 40) & 0xff) as u8,
        ((number >> 48) & 0xff) as u8,
        ((number >> 56) & 0xff) as u8,
        ((number >> 64) & 0xff) as u8,
        ((number >> 72) & 0xff) as u8,
        ((number >> 80) & 0xff) as u8,
        ((number >> 88) & 0xff) as u8,
        ((number >> 96) & 0xff) as u8,
        ((number >> 104) & 0xff) as u8,
        ((number >> 112) & 0xff) as u8,
        ((number >> 120) & 0xff) as u8,
    ]
}

pub fn u128_from_be(bytes: &[u8]) -> u128 {
    (bytes[15] as u128)
        + ((bytes[14] as u128) << 8)
        + ((bytes[13] as u128) << 16)
        + ((bytes[12] as u128) << 24)
        + ((bytes[11] as u128) << 32)
        + ((bytes[10] as u128) << 40)
        + ((bytes[9] as u128) << 48)
        + ((bytes[8] as u128) << 56)
        + ((bytes[7] as u128) << 64)
        + ((bytes[6] as u128) << 72)
        + ((bytes[5] as u128) << 80)
        + ((bytes[4] as u128) << 88)
        + ((bytes[3] as u128) << 96)
        + ((bytes[2] as u128) << 104)
        + ((bytes[1] as u128) << 112)
        + ((bytes[0] as u128) << 120)
}

pub fn u128_to_be(number: u128) -> [u8; 16] {
    [
        ((number >> 120) & 0xff) as u8,
        ((number >> 112) & 0xff) as u8,
        ((number >> 104) & 0xff) as u8,
        ((number >> 96) & 0xff) as u8,
        ((number >> 88) & 0xff) as u8,
        ((number >> 80) & 0xff) as u8,
        ((number >> 72) & 0xff) as u8,
        ((number >> 64) & 0xff) as u8,
        ((number >> 56) & 0xff) as u8,
        ((number >> 48) & 0xff) as u8,
        ((number >> 40) & 0xff) as u8,
        ((number >> 32) & 0xff) as u8,
        ((number >> 24) & 0xff) as u8,
        ((number >> 16) & 0xff) as u8,
        ((number >> 8) & 0xff) as u8,
        (number & 0xff) as u8,
    ]
}

pub fn u256_from_le(bytes: &[u8]) -> ethnum::U256 {
    ethnum::u256::from_words(u128_from_le(&bytes[16..32]), u128_from_le(&bytes[0..16]))
}

pub fn u256_to_le(number: ethnum::u256) -> [u8; 32] {
    let (mut temp_lo, mut temp_hi) = number.into_words();
    let mut bytes = [0u8; 32];

    for i in 0..16 {
        bytes[i] = (temp_hi & 0xFF) as u8;
        temp_hi >>= 8;
    }

    for i in 16..32 {
        bytes[i] = (temp_lo & 0xFF) as u8;
        temp_lo >>= 8;
    }
    bytes
}

pub fn u256_from_be(bytes: &[u8]) -> ethnum::U256 {
    ethnum::u256::from_words(u128_from_be(&bytes[0..16]), u128_from_be(&bytes[16..32]))
}

pub fn u256_to_be(number: ethnum::u256) -> [u8; 32] {
    let (mut temp_lo, mut temp_hi) = number.into_words();
    let mut bytes = [0u8; 32];

    for i in (0..16).rev() {
        bytes[i] = (temp_lo & 0xFF) as u8;
        temp_lo >>= 8;
    }

    for i in (16..32).rev() {
        bytes[i] = (temp_hi & 0xFF) as u8;
        temp_hi >>= 8;
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestHex(alloc::vec::Vec<u8>);
    impl ToHex for TestHex {
        fn get_bytes(&self) -> &[u8] {
            &self.0
        }
    }

    #[test]
    fn test_u16() {
        let number = 15685u16;
        let first_data = number.to_le_bytes();
        let second_data = u16_to_le(number);
        assert_eq!(first_data, second_data);
        let first: u16 = u16::from_le_bytes(first_data);
        let second: u16 = u16_from_le(&second_data);
        assert_eq!(first, second);
    }

    #[test]
    fn test_u32() {
        let number = 1568225u32;
        let first_data = number.to_le_bytes();
        let second_data = u32_to_le(number);
        assert_eq!(first_data, second_data);
        let first: u32 = u32::from_le_bytes(first_data);
        let second: u32 = u32_from_le(&second_data);
        assert_eq!(first, second);
    }

    #[test]
    fn test_u64() {
        let number = 15682255555u64;
        let first_data = number.to_le_bytes();
        let second_data = u64_to_le(number);
        assert_eq!(first_data, second_data);
        let first: u64 = u64::from_le_bytes(first_data);
        let second: u64 = u64_from_le(&second_data);
        assert_eq!(first, second);
    }

    #[test]
    fn test_u128() {
        let number = 055515682255555u128;
        let first_data = number.to_le_bytes();
        let second_data = u128_to_le(number);
        assert_eq!(first_data, second_data);
        let first: u128 = u128::from_le_bytes(first_data);
        let second: u128 = u128_from_le(&second_data);
        assert_eq!(first, second);

        let number = 055515633255555u128;
        let first_data = number.to_be_bytes();
        let second_data = u128_to_be(number);
        assert_eq!(first_data, second_data);
        let first: u128 = u128::from_be_bytes(first_data);
        let second: u128 = u128_from_be(&second_data);
        assert_eq!(first, second);
    }

    #[test]
    fn test_u256() {
        let number = ethnum::u256::new(128);
        let first_data = number.to_le_bytes();
        let second_data = u256_to_le(number);
        assert_eq!(first_data, second_data);

        let first: ethnum::u256 = ethnum::u256::from_le_bytes(first_data);
        let second: ethnum::u256 = u256_from_le(&second_data);
        assert_eq!(first, second);

        let number = ethnum::u256::new(128);
        let first_data = number.to_be_bytes();
        let second_data = u256_to_be(number);
        assert_eq!(first_data, second_data);

        let first: ethnum::u256 = ethnum::u256::from_be_bytes(first_data);
        let second: ethnum::u256 = u256_from_be(&second_data);
        assert_eq!(first, second);
    }

    #[test]
    fn test_to_hex() {
        let v = TestHex(alloc::vec![0xda, 0x02, 0xa1, 0x1f]);
        assert_eq!(v.to_hex(), alloc::string::String::from("0xda02a11f"));
        assert_eq!(
            to_hex(&255u8.to_be_bytes()),
            alloc::string::String::from("0xff")
        );
        assert_eq!(
            to_hex(&255u64.to_be_bytes()),
            alloc::string::String::from("0x00000000000000ff")
        );
    }
}
