#[cfg(not(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")))]
compile_error!("This crate can only be compiled for the x86_64-pc-windows-msvc target");

use core::ffi::c_void;
use core::mem::transmute;
use windows::core::{s, w, IUnknown, GUID, HRESULT};
use windows::Win32::Foundation::{HINSTANCE, HMODULE};
use windows::Win32::System::Console::{AllocConsole, GetConsoleMode, SetConsoleMode};
use windows::Win32::System::Console::{GetStdHandle, SetConsoleTitleW, SetStdHandle};
use windows::Win32::System::Console::{CONSOLE_MODE, ENABLE_VIRTUAL_TERMINAL_PROCESSING};
use windows::Win32::System::Console::{STD_ERROR_HANDLE, STD_OUTPUT_HANDLE};
use windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls;
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};

const VERSION: &str = env!("CARGO_PKG_VERSION");

static mut PROXY_FUNCTION: Option<fn()> = None;

#[no_mangle]
unsafe extern "system" fn DllMain(module: HMODULE, reason: u32, _: *mut c_void) -> bool {
    DisableThreadLibraryCalls(module).unwrap_unchecked();

    match reason {
        DLL_PROCESS_ATTACH => {
            CreateThread(
                None,
                0,
                Some(init),
                None,
                THREAD_CREATION_FLAGS::default(),
                None,
            )
            .unwrap();
        }
        DLL_PROCESS_DETACH => {}
        _ => (),
    }

    true
}

unsafe extern "system" fn init(_: *mut c_void) -> u32 {
    init_console();
    init_tracing();
    init_proxy();

    // let module = LoadLibraryA(s!("aard.dll")).unwrap();
    // let entry = GetProcAddress(module, s!("plugin")).unwrap();

    // let true_entry: unsafe fn() = transmute(entry);
    // true_entry();

    true as u32
}

unsafe fn init_proxy() {
    let module = LoadLibraryW(w!("C:\\Windows\\System32\\dinput8.dll")).unwrap();
    println!("[INIT] Loaded dinput8.dll at {}", module.0);

    PROXY_FUNCTION = Some(transmute(GetProcAddress(module, s!("DirectInput8Create"))));
    println!("[INIT] DirectInput8Create");
}

unsafe fn init_console() {
    AllocConsole().unwrap_unchecked();
    SetConsoleTitleW(w!("The Witcher 3: Console")).unwrap_unchecked();

    let stdout = GetStdHandle(STD_OUTPUT_HANDLE).unwrap();
    let stderr = GetStdHandle(STD_ERROR_HANDLE).unwrap();

    SetStdHandle(STD_OUTPUT_HANDLE, stdout).unwrap();
    SetStdHandle(STD_ERROR_HANDLE, stderr).unwrap();

    let mut stdout_mode = CONSOLE_MODE(0);
    GetConsoleMode(stdout, &mut stdout_mode).unwrap();
    SetConsoleMode(stdout, stdout_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING).unwrap();

    let mut stderr_mode = CONSOLE_MODE(0);
    GetConsoleMode(stderr, &mut stderr_mode).unwrap();
    SetConsoleMode(stderr, stderr_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING).unwrap();

    println!("The Witcher 3 - Axii {} - Plugin loader", VERSION);
}

fn init_tracing() {

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
    println!(
        "[CALL] DirectInput8Create ({:?}, {:?}, {:?}, {:?}, {:?})",
        hinst, dwversion, riidltf, ppvout, punkouter
    );
    type DirectInput8CreateFunc =
        extern "system" fn(HINSTANCE, u32, *const GUID, *mut *mut c_void, IUnknown) -> HRESULT;
    let func: DirectInput8CreateFunc = unsafe { transmute(PROXY_FUNCTION.unwrap()) };
    func(hinst, dwversion, riidltf, ppvout, punkouter)
}
