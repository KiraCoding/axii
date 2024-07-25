use core::ffi::c_void;
use std::mem::transmute_copy;
use std::ptr::null;
use std::sync::LazyLock;
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

/// Calculates the offset from the base address of the calling process (.exe file).
/// # Safety
/// If any of the following conditions are violated, the result is Undefined Behavior:
/// - The generic `F` must be an `unsafe extern "C" fn`.
/// - The computed offset, in bytes, cannot overflow an isize.
/// - The resulting function pointer from the computed offset must point to a function with the same signature as `F`.
/// # Examples
/// ```
/// let offset = 0x2843A0;
/// let add: unsafe extern "C" fn(u32, u32) -> u32 = resolve_fn(offset);
/// 
/// unsafe { assert_eq!(2, add(1, 1)); }
/// ```
#[must_use]
#[inline(always)]
pub unsafe fn resolve_fn<F>(offset: usize) -> F {
    struct Base(*mut c_void);
    unsafe impl Send for Base {}
    unsafe impl Sync for Base {}

    // SAFETY: `GetModuleHandleW(null)` returns a handle to the current process, which is (presumably) always valid for the lifetime of the process.
    // https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew
    static BASE: LazyLock<Base> =
        LazyLock::new(|| unsafe { Base(GetModuleHandleW(PCWSTR(null())).unwrap().0) });
    
    // SAFETY: The caller guarantees that `F` is an `unsafe extern "C" fn`.
    unsafe { transmute_copy(&BASE.0.add(offset)) }
}
