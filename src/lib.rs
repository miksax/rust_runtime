#![no_std]
#![feature(const_for)]
#![feature(const_trait_impl)]
#![feature(stmt_expr_attributes)]
#![feature(str_from_raw_parts)]
extern crate alloc;

pub mod blockchain;
pub mod constant;
pub mod contract;
pub mod cursor;
pub mod env;
pub mod error;
pub mod event;
pub mod math;
pub mod prelude;
pub mod storage;
pub mod types;
pub mod utils;

//pub mod mem;
pub mod tests;

use core::cell::RefCell;

use alloc::rc::Rc;
pub use env::*;
pub use ethnum;

pub use crate::constant::DEBUG;
#[cfg(not(target_arch = "wasm32"))]
pub use bitcoin;
pub use utils::*;

pub use contract::{
    op_20::{OP20Params, OP20Trait},
    ContractTrait,
};

pub static mut CONTRACT: Option<Rc<RefCell<dyn contract::ContractTrait>>> = None;

#[repr(transparent)]
pub struct WaPtr(u32);

impl From<&u32> for WaPtr {
    fn from(value: &u32) -> Self {
        Self(value as *const _ as *mut u8 as u32)
    }
}

impl From<&[u8]> for WaPtr {
    fn from(value: &[u8]) -> Self {
        Self(value as *const _ as *mut u8 as u32)
    }
}

impl From<&[u8; 32]> for WaPtr {
    fn from(value: &[u8; 32]) -> Self {
        Self(value as *const _ as *mut u8 as u32)
    }
}

fn log(text: &str) {
    unsafe {
        let bytes = text.as_bytes();
        env::global::log(bytes.as_ptr() as u32, bytes.len() as u32)
    };
}

#[cfg(not(test))]
#[cfg(not(feature = "std"))]
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable()
}

//#[cfg(target_arch = "wasm32")]
#[allow(static_mut_refs)]
#[export_name = "execute"]
pub unsafe fn execute(length: u32) -> u32 {
    if let Some(contract) = &mut CONTRACT {
        let mut contract = contract.borrow_mut();

        let call_data = contract.context().call_data(length as usize);

        match contract.execute(call_data) {
            Ok(result) => {
                if DEBUG {
                    log(&alloc::format!(
                        "Result of contract: {:?}",
                        result.clone().into_inner()
                    ));
                }

                env::global::exit(0, result.ptr(), result.size() as u32);
                0
            }
            Err(err) => {
                if DEBUG {
                    log(&alloc::format!(
                        "Contract failed with a error: {}",
                        err.as_str()
                    ));
                }
                1
            }
        }
    } else {
        if DEBUG {
            log("Contract is empty");
        }
        1
    }
}

#[cfg(target_arch = "wasm32")]
#[export_name = "onDeploy"]
#[allow(static_mut_refs)]
pub unsafe fn on_deploy(length: u32) -> u32 {
    if let Some(contract) = &mut CONTRACT {
        let mut contract = contract.borrow_mut();

        let call_data = contract.context().call_data(length as usize);
        let result = contract.on_deploy(call_data);

        0
    } else {
        1
    }
}
