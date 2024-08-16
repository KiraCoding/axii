use igni::program::program;
use crate::resolve;
use core::ffi::c_void;
use std::sync::LazyLock;

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
    pub(crate) this: *mut Function,
}

impl Function {
    pub fn new(hash: u32, function: extern "C" fn(u64, u64, u64)) -> Self {
        let memory = dbg!(alloc_func(0xC0, 0x10));
        println!("Allocation for func done");
        let this = unsafe { (FUNCTION_TABLE.new)(memory.cast(), hash, function as *const c_void) };
        println!("ctor done");
        Self { this }
    }
}

static ALLOC_FUNC: LazyLock<unsafe extern "C" fn(u32, u32) -> *mut c_void> =
    LazyLock::new(|| unsafe { dbg!(program().text().scan(&[]).unwrap()) });

fn alloc_func(size: u32, alignment: u32) -> *mut c_void {
    unsafe { (ALLOC_FUNC)(size, alignment) }
}
