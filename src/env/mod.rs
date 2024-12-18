use crate::mem::WaBuffer;
#[allow(unused_imports)]
use core::str::FromStr;

mod address;
mod global;
mod sha;
mod store;

pub use address::*;
pub use sha::*;
pub use store::*;

#[cfg(target_arch = "wasm32")]
pub fn log(text: &str) {
    unsafe {
        if let Ok(string) = WaBuffer::from_str(text) {
            global::log(string.ptr());
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn log(text: &str) {}

#[cfg(target_arch = "wasm32")]
pub fn emit(buffer: WaBuffer) {
    unsafe { global::emit(buffer.ptr()) }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn emit(buffer: WaBuffer) {}
