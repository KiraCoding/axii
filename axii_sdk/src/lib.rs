pub mod function;
pub mod names_pool;
pub mod rtti_system;

use core::ffi::{c_char, c_void};
use core::mem::transmute;
use std::env::current_dir;
use std::ffi::CString;
use std::mem::transmute_copy;
use std::os::windows::ffi::OsStrExt;
use std::sync::LazyLock;
use windows::core::{s, PCWSTR};
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};

pub use rtti_system::RTTISystem;

type Resolve = fn(*const c_char) -> *const c_void;

static RESOLVE: LazyLock<Resolve> = LazyLock::new(|| {
    let path: Vec<u16> = current_dir()
        .unwrap()
        .join("..\\whse\\axii.dll")
        .canonicalize()
        .unwrap()
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect();

    let module = unsafe { LoadLibraryW(PCWSTR(path.as_ptr())).unwrap() };
    let addr = unsafe { GetProcAddress(module, s!("resolve")).unwrap() };

    unsafe { transmute::<_, Resolve>(addr) }
});

pub(crate) fn resolve<T>(symbol: &str) -> T {
    let c_str = CString::new(symbol).unwrap();

    unsafe { transmute_copy(&RESOLVE(c_str.as_ptr())) }
}
