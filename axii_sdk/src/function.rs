use crate::resolve;
use core::ffi::{c_int, c_void};
use igni::program::program;
use std::sync::LazyLock;

static FUNCTION_TABLE: LazyLock<FunctionTable> = LazyLock::new(FunctionTable::init);

struct FunctionTable {
    new: unsafe extern "C-unwind" fn(*mut Function, c_int, *mut c_void) -> *mut Function,
}

impl FunctionTable {
    fn init() -> Self {
        let addr = unsafe {
            program()
                .text()
                .scan(&[
                    0x48, 0x89, 0x5c, 0x24, 0x08, 0x48, 0x89, 0x74, 0x24, 0x10, 0x57, 0x48, 0x83,
                    0xec, 0x20, 0x33, 0xf6, 0x48, 0x8d, 0x05, 0xc0, 0x5c, 0x58, 0x01, 0x48, 0x89,
                    0x01,
                ])
                .unwrap()
        };

        Self { new: dbg!(addr) }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Function([u8; 0xC0]);

impl Function {
    pub fn new(
        hash: u32,
        function: extern "C-unwind" fn(*mut c_void, *mut c_void, *mut c_void),
    ) -> Self {
        let memory = alloc_func(0xC0, 0x10);
        memset(memory, 0, 0xC0);
        unsafe {
            (FUNCTION_TABLE.new)(
                memory as *mut Function,
                hash as c_int,
                function as *mut c_void,
            )
        };
        Self([0; 192])
    }
}

static ALLOC_FUNC: LazyLock<unsafe extern "C" fn(usize, usize) -> *mut c_void> =
    LazyLock::new(|| unsafe {
        #[rustfmt::skip]
        let pattern = &[
            0x40, 0x53,                   // PUSH RBX
            0x48, 0x83, 0xEC, 0x30,       // SUB RSP, 0x30
            0x8B, 0xD9,                   // MOV EBX, size
            0x44, 0x8B, 0xCA,             // MOV R9D, alignment
            0x44, 0x8B, 0xC1,             // MOV R8D, size
            0x48, 0x8D, 0x54, 0x24, 0x20, // LEA alignment=>local_18,[RSP + 0x20]
        ];

        dbg!(program().text().scan(pattern).unwrap())
    });

fn alloc_func(size: usize, alignment: usize) -> *mut c_void {
    unsafe { (ALLOC_FUNC)(size, alignment) }
}

static MEMSET: LazyLock<unsafe extern "C" fn(*mut c_void, c_int, count: usize) -> *mut c_void> =
    LazyLock::new(|| unsafe {
        #[rustfmt::skip]
        let pattern = &[
            0x48, 0x8B, 0xC1,                         // MOV RAX, dest
            0x4C, 0x8B, 0xC9,                         // MOV R9, dest
            0x4C, 0x8D, 0x15, 0x03, 0xFF, 0x02, 0xFF, // R10, [IMAGE_DOS_HEADER__140000000]
        ];

        dbg!(program().text().scan(pattern).unwrap())
    });

fn memset(dest: *mut c_void, ch: c_int, count: usize) -> *mut c_void {
    unsafe { (MEMSET)(dest, ch, count) }
}
