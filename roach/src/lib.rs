use core::ffi::c_void;
use std::mem::transmute_copy;
use std::ptr::{null, null_mut};
use std::sync::Once;
use std::thread::sleep;
use std::time::Duration;
use windows::core::w;
use windows::{core::PCWSTR, Win32::System::LibraryLoader::GetModuleHandleW};

struct CFunction {}
struct NamesPool {}
struct CRTTISystem {}

unsafe extern "C" fn my_callback(p1: u64, p2: u64, p3: u64) {
    // Your callback implementation here
    println!("Callback called with p1: {}, p2: {}, p3: {}", p1, p2, p3);
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn plugin() {
    sleep(Duration::from_secs(120));

    let Allocator: unsafe extern "C" fn(size: usize, alignment: usize) -> *mut CFunction =
        resolve_fn(0x2846B0);
    let NamesPoolGet: unsafe extern "C" fn() -> *mut NamesPool = resolve_fn(0x2843A0);
    let NamesPoolAddEntry: unsafe extern "C" fn(*mut NamesPool, PCWSTR) -> u32 =
        resolve_fn(0x145A3A0);
    let CFunctionConstructor: unsafe extern "C" fn(
        *mut CFunction,
        u32,
        *mut c_void,
    ) -> *mut CFunction = resolve_fn(0x141496FA0);
    let CRTTISystemGet: unsafe extern "C" fn() -> *mut CRTTISystem = resolve_fn(0x285D60);
    let CRTTISystemRegisterGlobalFunction: unsafe extern "C" fn(*mut CRTTISystem, *mut CFunction) =
        resolve_fn(0x146A5f0);

    let names_pool = NamesPoolGet();
    let name_hash = NamesPoolAddEntry(names_pool, w!("TestFunction"));

    let memory = Allocator(0xC0, 0x10);
    let function = CFunctionConstructor(memory, name_hash, my_callback as *mut c_void);

    let rtti_system = CRTTISystemGet();
    CRTTISystemRegisterGlobalFunction(rtti_system, function);
}

pub unsafe fn resolve_fn<F>(offset: usize) -> F {
    static INIT: Once = Once::new();
    static mut BASE: *mut c_void = null_mut();

    INIT.call_once(|| unsafe { BASE = GetModuleHandleW(PCWSTR(null())).unwrap().0 });

    transmute_copy(&BASE.add(offset))
}
 