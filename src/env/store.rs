#[allow(unused_imports)]
use crate::{
    storage::{map::Map, StorageKey, StorageValue},
    WaBuffer,
};

#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
static mut STORAGE: Map<StorageKey, StorageValue> = Map::new();

#[cfg(target_arch = "wasm32")]
pub fn pointer_store(key: &StorageKey, value: &StorageValue) -> Result<bool, crate::error::Error> {
    let mut buffer = WaBuffer::new(64, 1)?;
    let mut cursor = buffer.cursor();

    cursor.write_bytes(key)?;
    cursor.write_bytes(value.bytes())?;

    unsafe {
        WaBuffer::from_raw(super::global::store(buffer.ptr()))
            .cursor()
            .read_bool()
    }
}

/**
 * For unit test only
 */
#[cfg(not(target_arch = "wasm32"))]
pub fn pointer_store(key: &StorageKey, value: &StorageValue) -> Result<bool, crate::error::Error> {
    unsafe { STORAGE.insert(*key, *value) };
    Ok(true)
}

#[cfg(target_arch = "wasm32")]
pub fn pointer_load(key: &StorageKey) -> Result<StorageValue, crate::error::Error> {
    let mut buffer = WaBuffer::new(32, 1)?;
    let mut cursor = buffer.cursor();

    cursor.write_bytes(key)?;
    unsafe {
        let value: StorageValue = WaBuffer::from_raw(super::global::load(buffer.ptr()))
            .data()
            .into();
        Ok(value)
    }
}
#[cfg(not(target_arch = "wasm32"))]
#[allow(static_mut_refs)]
pub fn pointer_load(key: &StorageKey) -> Result<StorageValue, crate::error::Error> {
    unsafe { Ok(*STORAGE.get(key).unwrap_or(&StorageValue::ZERO)) }
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(static_mut_refs)]
pub fn pointer_storage_reset() {
    unsafe {
        STORAGE.clear();
    }
}

/*
pub fn pointer_next_greater_than(
    target_pointer: &StorageKey,
    value_at_least: &StorageValue,
    lte: bool,
) -> Result<StorageValue, crate::error::Error> {
    let mut buffer = WaBuffer::new(64, 1)?;
    let mut cursor = buffer.cursor();
    cursor.write_bytes(target_pointer)?;
    cursor.write_bytes(value_at_least.bytes())?;
    cursor.write_bool(lte)?;

    unsafe {
        Ok(
            WaBuffer::from_raw(super::global::nextPointerGreaterThan(buffer.ptr()))
                .data()
                .into(),
        )
    }
}
 */
