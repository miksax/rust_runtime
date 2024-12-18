use crate::mem::WaPtr;

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "env")]
extern "C" {
    #[allow(dead_code)]
    pub fn load(ptr: WaPtr) -> WaPtr;
    #[allow(dead_code)]
    pub fn store(ptr: WaPtr) -> WaPtr;

    #[allow(dead_code)]
    pub fn nextPointerGreaterThan(ptr: WaPtr) -> WaPtr;

    #[allow(dead_code)]
    pub fn deploy(ptr: WaPtr) -> WaPtr;
    #[allow(dead_code)]
    pub fn deployFromAddress(ptr: WaPtr) -> WaPtr;

    #[allow(dead_code)]
    pub fn call(ptr: WaPtr) -> WaPtr;
    #[allow(dead_code)]
    pub fn log(ptr: WaPtr);
    #[allow(dead_code)]
    pub fn emit(ptr: WaPtr);
    #[allow(dead_code)]
    pub fn encodeAddress(ptr: WaPtr) -> WaPtr;
    #[allow(dead_code)]
    pub fn validateBitcoinAddress(ptr: WaPtr) -> WaPtr;
    #[allow(dead_code)]
    pub fn sha256(ptr: WaPtr) -> WaPtr;
    #[allow(dead_code)]
    pub fn ripemd160(ptr: WaPtr) -> WaPtr;
    #[allow(dead_code)]
    pub fn inputs() -> WaPtr;
    #[allow(dead_code)]
    pub fn outputs() -> WaPtr;
}
