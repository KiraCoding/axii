#![no_std]

#[cfg(not(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")))]
compile_error!("This crate can only be compiled for the x86_64-pc-windows-msvc target");

use core::ffi::c_void;
use core::mem::transmute;
use windows::core::{s, w, IUnknown, GUID, HRESULT};
use windows::Win32::Foundation::{HINSTANCE, HMODULE};
use windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls;
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};

type DirectInput8CreateFunc =
    extern "system" fn(HINSTANCE, u32, *const GUID, *mut *mut c_void, IUnknown) -> HRESULT;

static mut PROXY_FUNCTION: Option<DirectInput8CreateFunc> = None;

#[no_mangle]
unsafe extern "system" fn DllMain(module: HMODULE, reason: u32, _: *mut c_void) -> bool {
    DisableThreadLibraryCalls(module).unwrap_unchecked();

    if reason == DLL_PROCESS_ATTACH {
        CreateThread(
            None,
            0,
            Some(init),
            None,
            THREAD_CREATION_FLAGS::default(),
            None,
        )
        .unwrap_unchecked();
    }

    true
}

unsafe extern "system" fn init(_: *mut c_void) -> u32 {
    let module = LoadLibraryW(w!("C:\\Windows\\System32\\dinput8.dll")).unwrap_unchecked();
    PROXY_FUNCTION = Some(transmute(GetProcAddress(module, s!("DirectInput8Create"))));

    let loader = LoadLibraryW(w!("..\\whse\\axii.dll")).unwrap_unchecked();
    let entry = GetProcAddress(loader, s!("loader")).unwrap_unchecked();

    transmute::<_, unsafe fn()>(entry)();

    true as u32
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DirectInput8Create(
    hinst: HINSTANCE,
    dwversion: u32,
    riidltf: *const GUID,
    ppvout: *mut *mut c_void,
    punkouter: IUnknown,
) -> HRESULT {
    PROXY_FUNCTION.unwrap_unchecked()(hinst, dwversion, riidltf, ppvout, punkouter)
}
