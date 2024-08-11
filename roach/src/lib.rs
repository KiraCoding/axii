#[cfg(not(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")))]
compile_error!("This crate can only be compiled for the x86_64-pc-windows-msvc target");

use igni::{hook::hook, program::program};

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn plugin() {
    let program = program();
    dbg!(program.base());

    let sig = &[
        0x48, 0x89, 0x5c, 0x24, 0x10, 0x57, 0x48, 0x83, 0xEC, 0x20, 0xBA, 0x10, 0x00, 0x00, 0x00,
    ];

    let addr = dbg!(program.scan::<unsafe extern "C" fn()>(sig).unwrap());

    hook(addr, || println!("Hook worked!"));
}
