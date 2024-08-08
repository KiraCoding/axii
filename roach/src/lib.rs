use igni::{hook::Hookable, program::program};

#[cfg(not(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")))]
compile_error!("This crate can only be compiled for the x86_64-pc-windows-msvc target");
// pub mod names_pool;
// pub mod rtti_system;

// use core::ffi::c_void;
// use std::thread::sleep;
// use std::time::Duration;
// use transcend::ptr::resolve_fn;
// use windows::core::w;
// use windows::core::PCWSTR;

// struct CRTTISystem {}
// struct CNamesPool {}
// struct CFunction {}

// unsafe extern "C" fn my_callback(p1: u64, p2: u64, p3: u64) {
//     // Your callback implementation here
//     println!("Callback called with p1: {}, p2: {}, p3: {}", p1, p2, p3);
// }

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn plugin() {
    let program = program();
    dbg!(program.base());
    dbg!(program.sections());

    let result = program
        .scan(&[
            0x48, 0x89, 0x5c, 0x24, 0x10, 0x57, 0x48, 0x83, 0xEC, 0x20, 0xBA, 0x10, 0x00, 0x00,
            0x00,
        ])
        .unwrap();

    let result: unsafe extern "cdecl" fn() = std::mem::transmute(result);

    dbg!(result);

    result.hook(|| {
        println!("Hook")
    });

    // sleep(Duration::from_secs(120));

    // let Allocator: unsafe extern "C" fn(size: usize, alignment: usize) -> *mut CFunction =
    //     resolve_fn(0x2846B0);

    // let NamesPoolGet: unsafe extern "C" fn() -> *mut CNamesPool = resolve_fn(0x2843A0);

    // let NamesPoolAddEntry: unsafe extern "thiscall" fn(*mut CNamesPool, PCWSTR) -> u32 =
    //     resolve_fn(0x145A3A0);

    // let CFunctionConstructor: unsafe extern "thiscall" fn(
    //     *mut CFunction,
    //     u32,
    //     *mut c_void,
    // ) -> *mut CFunction = resolve_fn(0x141496FA0);

    // let CRTTISystemGet: unsafe extern "C" fn() -> *mut CRTTISystem = resolve_fn(0x285D60);

    // let CRTTISystemRegisterGlobalFunction: unsafe extern "thiscall" fn(
    //     *mut CRTTISystem,
    //     *mut CFunction,
    // ) = resolve_fn(0x146A5f0);

    // let names_pool = NamesPoolGet();
    // let name_hash = NamesPoolAddEntry(names_pool, w!("TestFunction"));

    // let memory = Allocator(0xC0, 0x10);
    // let function = CFunctionConstructor(memory, name_hash, my_callback as *mut c_void);

    // let rtti_system = CRTTISystemGet();
    // CRTTISystemRegisterGlobalFunction(rtti_system, function);
}
