use core::ffi::c_void;
use std::alloc::{alloc, Layout};
use std::sync::LazyLock;
use crate::resolve;

static FUNCTION_TABLE: LazyLock<FunctionTable> = LazyLock::new(FunctionTable::init);

struct FunctionTable {
    new: unsafe extern "C" fn(*mut Function, u32, *const c_void) -> *mut Function,
}

impl FunctionTable {
    fn init() -> Self {
        Self {
            new: dbg!(resolve("CFunction::CFunction")),
        }
    }
}

#[derive(Debug)]
pub struct Function {
    layout: Layout,
    pub(crate) this: *mut Function,
}

impl Function {
    pub fn new(name_hash: u32, function: extern "C" fn(*const c_void, *const c_void, u64)) -> Self {
        let layout = Layout::from_size_align(0xC0, 0x10).unwrap();
        let memory = unsafe { alloc(layout) };

        let this =
            unsafe { (FUNCTION_TABLE.new)(memory.cast(), name_hash, function as *const c_void) };

        Self { layout, this }
    }
}
