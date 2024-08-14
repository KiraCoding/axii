use std::{mem::ManuallyDrop, sync::LazyLock};

use crate::resolve;
use igni::program::program;

static NAMES_POOL_TABLE: LazyLock<NamesPoolTable> = LazyLock::new(NamesPoolTable::init);

struct NamesPoolTable {
    get: unsafe extern "C" fn() -> *mut NamesPool,
    add_entry: unsafe extern "C" fn(*mut NamesPool, *const u16) -> *const u32,
}

impl NamesPoolTable {
    fn init() -> Self {
        #[rustfmt::skip]
        let pattern = &[
            0x48, 0x83, 0xEC, 0x28,                  // SUB RSP, 0x28
            0x48, 0x8B, 0x05, 0x7D, 0x10, 0x55, 0x05 // RAX qword ptr [null_0000000000000000h_1457d5428]
        ];

        Self {
            get: dbg!(unsafe { program().text().scan(pattern).unwrap() }),
            add_entry: dbg!(resolve("CNamesPool::AddEntry")),
        }
    }
}

pub struct NamesPool {
    this: *mut NamesPool,
}

impl NamesPool {
    fn get() -> Self {
        let this = unsafe { (NAMES_POOL_TABLE.get)() };
        Self { this }
    }

    pub fn add_entry(name: &str) -> Hash {
        let name_wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
        unsafe {
            (NAMES_POOL_TABLE.add_entry)(Self::get().this, ManuallyDrop::new(name_wide).as_ptr())
        }
    }
}

pub type Hash = *const u32;
