#[cfg(not(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")))]
compile_error!("This crate can only be compiled for the x86_64-pc-windows-msvc target");

use aard::names_pool::NamesPool;

#[no_mangle]
pub unsafe extern "system" fn plugin() {
    let name_hash = NamesPool::add_entry("EngineTimeToFloat");
    let found = NamesPool::find_text_ansi(name_hash).unwrap();
    println!("Name: {}", found);
}
