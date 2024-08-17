#[cfg(not(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")))]
compile_error!("This crate can only be compiled for the x86_64-pc-windows-msvc target");

use core::ffi::c_void;
use axii_sdk::names_pool::NamesPool;
use axii_sdk::{Function, RTTISystem};
use igni::program::program;

extern "C-unwind" fn test(_p1: *mut c_void, _script_stack_frame: *mut c_void, _return: *mut c_void) {
    println!("Hello")
}

#[no_mangle]
pub unsafe extern "system" fn plugin() {
    #[rustfmt::skip]
    let pattern = &[
        0x48, 0x89, 0x5C, 0x24, 0x10, // MOV qword ptr [RSP + local_res10], RBX
        0x48, 0x89, 0x7C, 0x24, 0x18, // MOV qword ptr [RSP + local_res18], RDI
        0x55,                         // PUSH RBP
        0x48, 0x8B, 0xEC,             // MOV RBP, RSP
        0x48, 0x83, 0xEC, 0x20,       // SUB RSP, 0x20
        0xBA, 0x10, 0x00, 0x00, 0x00, // EDX, 0x10
        0xB9, 0xC0, 0x00, 0x00, 0x00, // ECX, 0xC0
        0xE8, 0x7F, 0xF5, 0x24, 0xFE, // CALL AllocateFunc?
    ];

    let addr = dbg!(program().text().scan(pattern).unwrap());

    minhook::MinHook::create_hook(addr, hook as _).unwrap();
    minhook::MinHook::enable_all_hooks().unwrap();
    println!("HOOKED")
}

fn hook() {
    let hash = dbg!(NamesPool::add_entry("TestFunction"));
    let function = Function::new(hash, test);

    dbg!(function);
    println!("Done");
}
