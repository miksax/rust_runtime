use alloc::alloc::{alloc, Layout};
use alloc::slice;
use core::ptr::NonNull;
use core::str::FromStr;

use crate::cursor::Cursor; // Import format and String from alloc

extern crate alloc;

pub type WaPtr = u32;

pub struct WaCell {
    pub mm_info: u64,
    pub gc_info1: u64,
    pub gc_info2: u64,
    pub rt_id: u32,
    pub rt_size: u32,
}

impl WaCell {
    const fn size() -> WaPtr {
        size_of::<Self>() as WaPtr
    }
    const fn usize() -> usize {
        size_of::<Self>()
    }
    const fn layout(size: usize) -> Layout {
        unsafe { Layout::from_size_align_unchecked(size + WaCell::usize(), 1) }
    }
    pub fn new(size: usize, id: u32) -> &'static mut WaCell {
        unsafe {
            let layout = WaCell::layout(size);
            let ptr = alloc(layout);
            let cell = NonNull::<WaCell>::new_unchecked(ptr.cast()).as_mut();

            cell.gc_info1 = 0;
            cell.mm_info = ptr as u64;
            cell.rt_id = id;
            cell.rt_size = size as u32;
            cell
        }
    }

    pub fn new_data(id: u32, data: &[u8]) -> &'static mut WaCell {
        let cell = WaCell::new(data.len(), id);
        cell.data_mut().copy_from_slice(data); // Use `copy_from_slice` instead of `write`
        cell
    }

    pub fn from_raw(ptr: WaPtr) -> &'static mut WaCell {
        unsafe { NonNull::<WaCell>::new_unchecked((ptr - WaCell::size()) as *mut WaCell).as_mut() }
    }

    #[inline]
    pub fn ptr(&self) -> WaPtr {
        (self as *const Self as *const u8 as WaPtr) + WaCell::size()
    }

    pub fn data(&self) -> &'static [u8] {
        unsafe { slice::from_raw_parts(self.ptr() as *const u8, self.rt_size as usize) }
    }

    pub fn data_mut(&mut self) -> &'static mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr() as *mut u8, self.rt_size as usize) }
    }

    pub fn cursor(&mut self) -> super::cursor::Cursor {
        super::cursor::Cursor::from_slice(self.data_mut())
    }
}

pub struct WaBuffer {
    buffer: &'static mut WaCell,
    pointer: &'static mut WaCell,
}

impl Clone for WaBuffer {
    fn clone(&self) -> Self {
        WaBuffer {
            buffer: WaCell::from_raw(self.buffer.ptr()),
            pointer: WaCell::from_raw(self.pointer.ptr()),
        }
    }
}

impl WaBuffer {
    pub fn new(size: usize, id: u32) -> Result<WaBuffer, crate::error::Error> {
        let buffer = WaCell::new(size, 1);
        let pointer = WaCell::new(12, id);
        let mut cursor = pointer.cursor();
        cursor.write_u32_le(&buffer.ptr())?;
        cursor.write_u32_le(&buffer.ptr())?;
        cursor.write_u32_le(&(size as u32))?;
        Ok(WaBuffer { pointer, buffer })
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<WaBuffer, crate::error::Error> {
        let buffer = WaCell::new_data(1, bytes);
        let pointer = WaCell::new(12, 2);
        let mut cursor = pointer.cursor();
        cursor.write_u32_le(&buffer.ptr())?;
        cursor.write_u32_le(&buffer.ptr())?;
        cursor.write_u32_le(&(bytes.len() as u32))?;
        Ok(WaBuffer { pointer, buffer })
    }
    pub fn from_raw(ptr: WaPtr) -> WaBuffer {
        let pointer = WaCell::from_raw(ptr);
        let mut cursor = pointer.cursor();
        let buffer_ptr = cursor.read_u32_le_unchecked();
        let buffer = WaCell::from_raw(buffer_ptr);
        WaBuffer { pointer, buffer }
    }

    pub fn ptr(&self) -> WaPtr {
        self.pointer.ptr()
    }

    pub fn data(&self) -> &'static [u8] {
        self.buffer.data()
    }

    pub fn cursor(&mut self) -> Cursor {
        Cursor::from_slice(self.buffer.data_mut())
    }

    /// # Safety
    ///
    /// Cast memory to given Type.
    /// It is very unsafe and works only on WASM32 arch due to memory align and type sizes
    #[cfg(target_arch = "wasm32")]
    pub unsafe fn into_type<T: Sized>(&self) -> &'static mut T {
        if core::mem::size_of::<T>() <= self.buffer.rt_size as usize {
            NonNull::new_unchecked(self.buffer.ptr() as *mut T).as_mut()
        } else {
            panic!("Cannot map larger objects");
        }
    }
}

impl FromStr for WaBuffer {
    type Err = crate::error::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let len = bytes.len() as u16;
        let str_data = len
            .to_le_bytes()
            .iter()
            .chain(bytes)
            .cloned()
            .collect::<alloc::vec::Vec<u8>>();
        WaBuffer::from_bytes(&str_data)
    }
}

#[export_name = "__new"]
pub fn new(size: usize, id: u32) -> WaPtr {
    WaCell::new(size, id).ptr()
}

#[export_name = "__pin"]
pub fn pin(ptr: WaPtr) -> WaPtr {
    ptr
}

#[export_name = "__unpin"]
pub fn unpin(_ptr: WaPtr) {}
