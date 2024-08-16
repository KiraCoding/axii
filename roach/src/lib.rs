#[cfg(not(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")))]
compile_error!("This crate can only be compiled for the x86_64-pc-windows-msvc target");

use axii_sdk::names_pool::NamesPool;
use axii_sdk::{function::Function, RTTISystem};

#[no_mangle]
pub unsafe extern "system" fn plugin() {
    let hash = NamesPool::add_entry("TestFunction");
    let function = Function::new(hash, |a, b, c| {});

    dbg!(function);
    println!("Done");
}
