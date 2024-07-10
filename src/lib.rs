#[cfg(not(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")))]
compile_error!("This crate can only be compiled for the x86_64-pc-windows-msvc target");

use core::ffi::c_void;
use core::mem::transmute;
use windows::core::{s, IUnknown, GUID};
use windows::Win32::Foundation::{FreeLibrary, HINSTANCE, HMODULE};
use windows::Win32::System::Console::{AllocConsole, GetConsoleMode, SetConsoleMode};
use windows::Win32::System::Console::{GetStdHandle, SetConsoleTitleA, SetStdHandle};
use windows::Win32::System::Console::{CONSOLE_MODE, ENABLE_VIRTUAL_TERMINAL_PROCESSING};
use windows::Win32::System::Console::{STD_ERROR_HANDLE, STD_OUTPUT_HANDLE};
use windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls;
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};

type DWORD = u32;
type LPVOID = *mut c_void;
type DirectInput8CreateFunc = extern "system" fn(HINSTANCE, DWORD, GUID, LPVOID, IUnknown);

const VERSION: &str = env!("CARGO_PKG_VERSION");

static mut MODULE_PROXY: Option<HMODULE> = None;
static mut MODULE_SYSTEM: Option<HMODULE> = None;

static mut PROXY_FUNCTION: Option<fn()> = None;

#[no_mangle]
unsafe extern "system" fn DllMain(module: HMODULE, reason: u32, _: *mut c_void) -> bool {
    DisableThreadLibraryCalls(module).unwrap_unchecked();
    MODULE_PROXY = Some(module);

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
        DLL_PROCESS_DETACH => {
            FreeLibrary(MODULE_SYSTEM.unwrap()).unwrap();
        }
        _ => (),
    }

    true
}

unsafe extern "system" fn init(_: *mut c_void) -> u32 {
    init_console();
    println!("The Witcher 3 - Axii {} - Plugin loader", VERSION);

    // Load the true system `dinput8.dll`
    MODULE_SYSTEM = Some(LoadLibraryA(s!("C:\\Windows\\System32\\dinput8.dll")).unwrap());
    println!("[INIT] Loaded dinput8.dll [{:?}]", MODULE_SYSTEM.unwrap());

    // Load the true `DirectInput8Create`
    PROXY_FUNCTION = Some(transmute(GetProcAddress(
        MODULE_SYSTEM.unwrap(),
        s!("DirectInput8Create"),
    )));
    println!("[INIT] DirectInput8Create");

    true as u32
}

unsafe fn init_console() {
    AllocConsole().unwrap_unchecked();
    SetConsoleTitleA(s!("The Witcher 3: Console")).unwrap();

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
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DirectInput8Create(
    hinst: HINSTANCE,
    dwVersion: DWORD,
    riidltf: GUID,
    ppvOut: LPVOID,
    punkOuter: IUnknown,
) {
    println!(
        "[CALL] DirectInput8Create ({:?}, {:?}, {:?}, {:?}, {:?})",
        hinst, dwVersion, riidltf, ppvOut, punkOuter
    );

    let func: DirectInput8CreateFunc = unsafe { transmute(PROXY_FUNCTION.unwrap()) };
    func(hinst, dwVersion, riidltf, ppvOut, punkOuter);
}
