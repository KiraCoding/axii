use std::sync::LazyLock;
use transcend::ptr::resolve_fn;
use windows::core::PCWSTR;

static NAMES_POOL_TABLE: LazyLock<NamesPoolTable> = LazyLock::new(|| NamesPoolTable {
    get: unsafe { resolve_fn(0x2843A0) },
    add_entry: unsafe { resolve_fn(0x145A3A0) },
});

#[derive(Debug)]
struct NamesPoolTable {
    get: unsafe extern "fastcall" fn() -> *mut NamesPool,
    add_entry: unsafe extern "thiscall" fn(*mut NamesPool, PCWSTR) -> *mut u32,
}

#[derive(Debug, Clone, Copy)]
pub struct NamesPool {}

impl NamesPool {
    pub fn get() -> Self {
        unsafe { *(NAMES_POOL_TABLE.get)() }
    }

    pub fn add_entry(&self, name: PCWSTR) -> *mut u32 {
        unsafe { (NAMES_POOL_TABLE.add_entry)(self as *const _ as *mut _, name) }
    }
}
