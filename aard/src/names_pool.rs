use core::ffi::c_char;
use core::mem::ManuallyDrop;
use core::slice::from_raw_parts;
use std::ffi::CStr;
use std::sync::LazyLock;

use crate::resolve;
use igni::program::program;

static NAMES_POOL_TABLE: LazyLock<NamesPoolTable> = LazyLock::new(NamesPoolTable::init);

struct NamesPoolTable {
    get: unsafe extern "C" fn() -> *mut NamesPool,
    add_entry: unsafe extern "C" fn(*mut NamesPool, *const u16) -> u32,
    find_text: unsafe extern "C" fn(*mut NamesPool, u32) -> *const u16,
    find_text_ansi: unsafe extern "C" fn(*mut NamesPool, u32) -> *const c_char,
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
            find_text: dbg!(resolve("CNamesPool::FindText")),
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

    pub fn add_entry(name: &str) -> u32 {
        let name_wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
        unsafe {
            (NAMES_POOL_TABLE.add_entry)(Self::get().this, ManuallyDrop::new(name_wide).as_ptr())
        }
    }

    pub fn find_text(key: u32) -> Option<String> {
        unsafe {
            let name_wide_ptr = (NAMES_POOL_TABLE.find_text)(Self::get().this, key);

            if name_wide_ptr.is_null() {
                return None;
            }

            let len = (0..).take_while(|&i| *name_wide_ptr.add(i) != 0).count();

            let utf16_slice = from_raw_parts(name_wide_ptr, len);
            Some(String::from_utf16(utf16_slice).unwrap())
        }
    }

    pub fn find_text_ansi(key: u32) -> Option<&str> {
        let ansi_str_ptr = unsafe { (NAMES_POOL_TABLE.find_text_ansi)(Self::get().this, key) };

        if ansi_str_ptr.is_null() {
            return None;
        }

        let c_str = unsafe { CStr::from_ptr(ansi_str_ptr) };

        Some(c_str.to_str().unwrap())
    }
}
