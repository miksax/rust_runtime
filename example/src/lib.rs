#![no_std]
extern crate alloc;

#[allow(unused_imports)]
use rust_runtime::prelude::{ContractTrait, WaBuffer, WaPtr};
pub mod contract;

#[allow(dead_code, static_mut_refs)]
static mut CONTRACT: contract::Contract = contract::Contract::new();

#[cfg(target_arch = "wasm32")]
use lol_alloc::LeakingPageAllocator;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: LeakingPageAllocator = LeakingPageAllocator;

#[cfg(target_arch = "wasm32")]
#[allow(static_mut_refs)]
#[export_name = "execute"]
pub unsafe fn execute(ptr: WaPtr) -> WaPtr {
    match CONTRACT.execute(WaBuffer::from_raw(ptr).cursor()) {
        Ok(buffer) => buffer.ptr(),
        Err(err) => {
            rust_runtime::log(err.as_str());
            panic!("Error occured")
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[export_name = "onDeploy"]
#[allow(static_mut_refs)]
pub unsafe fn on_deploy(ptr: WaPtr) {
    CONTRACT.on_deploy(WaBuffer::from_raw(ptr).cursor());
}

#[cfg(target_arch = "wasm32")]
#[export_name = "setEnvironment"]
#[allow(static_mut_refs)]
pub unsafe fn set_environment(ptr: WaPtr) {
    let buffer = WaBuffer::from_raw(ptr);
    let environment: &mut rust_runtime::blockchain::Environment = buffer.into_type();
    CONTRACT.set_environment(environment);
}
