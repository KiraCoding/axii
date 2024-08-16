use crate::resolve;
use core::cell::UnsafeCell;
use core::ffi::c_void;
use std::alloc::{alloc, Layout};
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
    layout: Layout,
    pub(crate) this: *mut Function,
}

impl Function {
    pub fn new<F>(hash: u32, function: F) -> Self
    where
        F: FnMut(u64, u64, u64),
    {
        let layout = Layout::from_size_align(0xC0, 0x10).unwrap();
        let memory = unsafe { alloc(layout) };

        let closure = Box::into_raw(Box::new(function));

        thread_local! {
            static CTX: UnsafeCell<*mut c_void> = UnsafeCell::new(std::ptr::null_mut());
        }

        CTX.with(|ctx| unsafe {
            *ctx.get() = closure as *mut c_void;
        });

        unsafe extern "C" fn trampoline<F>(a: u64, b: u64, c: u64)
        where
            F: FnMut(u64, u64, u64),
        {
            CTX.with(|ctx| {
                (*(*ctx.get() as *mut F))(a, b, c);
            });
        }

        let this =
            unsafe { (FUNCTION_TABLE.new)(memory.cast(), hash, trampoline::<F> as *const c_void) };

        Self { layout, this }
    }
}
