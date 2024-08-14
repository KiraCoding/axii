#[cfg(not(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")))]
compile_error!("This crate can only be compiled for the x86_64-pc-windows-msvc target");

use aard::{function::Function, names_pool::NamesPool, rtti_system::RTTISystem};
use core::ffi::c_void;

extern "C" fn abcd(p0: *const c_void, script_stack_frame: *const c_void, ret: u64) {}

#[no_mangle]
pub unsafe extern "system" fn plugin() {
    let name_hash = NamesPool::add_entry("abcd");
    dbg!(name_hash);
    let function = Function::new(name_hash, abcd);
    dbg!(&function);
    println!("Func done alloc");

    RTTISystem::register_global_function(function);
    println!("Registered")
}
