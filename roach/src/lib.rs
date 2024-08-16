#[cfg(not(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")))]
compile_error!("This crate can only be compiled for the x86_64-pc-windows-msvc target");

use axii_sdk::names_pool::NamesPool;
use axii_sdk::{Function, RTTISystem};

extern "C" fn test(_p1: u64, _script_stack_frame: u64, _return: u64) {
    println!("Hello")
}

#[no_mangle]
pub unsafe extern "system" fn plugin() {
    let hash = NamesPool::add_entry("TestFunction");
    let function = Function::new(hash, test);

    dbg!(function);
    println!("Done");
}
