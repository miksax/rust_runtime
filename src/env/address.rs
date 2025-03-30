#[cfg(target_arch = "wasm32")]
pub fn validate_bitcoin_address(address: &str) -> Result<bool, crate::error::Error> {
    let mut buffer = crate::WaBuffer::new(address.len(), 1)?;
    let mut cursor = buffer.cursor();
    cursor.write_string(address)?;

    unsafe {
        crate::WaBuffer::from_raw(super::global::validateBitcoinAddress(buffer.ptr()))
            .cursor()
            .read_bool()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn validate_bitcoin_address(address: &str) -> Result<bool, crate::error::Error> {}
