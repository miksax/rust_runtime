use crate::mem::WaBuffer;
use core::str::FromStr;

mod address;
mod global;
mod sha;
mod store;

pub use address::*;
pub use sha::*;
pub use store::*;

pub fn log(text: &str) {
    unsafe {
        let string = WaBuffer::from_str(text).unwrap();
        global::log(string.ptr());
    }
}

pub fn emit(buffer: WaBuffer) {
    unsafe { global::emit(buffer.ptr()) }
}
