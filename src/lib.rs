#[cfg(not(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")))]
compile_error!("This crate can only be compiled for the x86_64-pc-windows-msvc target");

mod plugin;
mod registry;

use core::ffi::c_void;
use core::mem::transmute;
use registry::Registry;
use std::ffi::CString;
use std::fs;
use std::path::Path;
use windows::core::{s, IUnknown, GUID, PCSTR};
use windows::Win32::Foundation::{FreeLibrary, HINSTANCE, HMODULE};
use windows::Win32::System::Console::{AllocConsole, GetConsoleMode, SetConsoleMode};
use windows::Win32::System::Console::{GetStdHandle, SetConsoleTitleA, SetStdHandle};
use windows::Win32::System::Console::{CONSOLE_MODE, ENABLE_VIRTUAL_TERMINAL_PROCESSING};
use windows::Win32::System::Console::{STD_ERROR_HANDLE, STD_OUTPUT_HANDLE};
use windows::Win32::System::LibraryLoader::{DisableThreadLibraryCalls, GetModuleHandleA};
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

// #[repr(C)]
// #[derive(Debug)]
// struct CRTTISystem([u8; 0xF0]);

unsafe extern "system" fn init(_: *mut c_void) -> u32 {
    init_console();
    init_proxy();
    init_plugins();

    Registry::new();

    // let base = GetModuleHandleA(PCSTR::null()).unwrap().0;
    // let c_rtti_system_get_offset = 0x285D60;

    // let addr = base + c_rtti_system_get_offset;

    // println!("[INFO] Base: {:?}", base);

    // type CRTTISystemGet = unsafe extern "C" fn() -> *const [u8; 0xF0];
    // let func: CRTTISystemGet = transmute(addr);

    // let ptr = func();

    // if ptr.is_null() {
    //     println!("Function returned null ptr");
    // } else {
    //     let system = &*ptr;
    //     println!("CRTTISystem {:?}", system);
    // }

    true as u32
}

unsafe fn init_plugins() {
    let plugins_dir = "../axii/plugins/";
    let mut plugins_found = false;

    if let Ok(entries) = fs::read_dir(plugins_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                if extension.to_lowercase() == "dll" {
                    load_dll(&path);
                    plugins_found = true;
                }
            }
        }
    }

    if !plugins_found {
        println!("[INFO] No plugins found in {:?}", plugins_dir);
    }
}

unsafe fn load_dll(path: &Path) {
    let path_str = path.to_str().expect("Invalid DLL path");
    let path_cstr = CString::new(path_str).expect("CString conversion failed");

    match LoadLibraryA(PCSTR(path_cstr.as_ptr() as *const u8)) {
        Ok(_module) => println!("[INIT] Loaded plugin: {:?}", path),
        Err(err) => println!("[ERROR] Failed to load plugin {:?}: {:?}", path, err),
    }
}

unsafe fn init_proxy() {
    MODULE_SYSTEM = Some(LoadLibraryA(s!("C:\\Windows\\System32\\dinput8.dll")).unwrap());
    println!("[INIT] Loaded dinput8.dll at {}", MODULE_SYSTEM.unwrap().0);

    PROXY_FUNCTION = Some(transmute(GetProcAddress(
        MODULE_SYSTEM.unwrap(),
        s!("DirectInput8Create"),
    )));
    println!("[INIT] DirectInput8Create");
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

    println!("The Witcher 3 - Axii {} - Plugin loader", VERSION);
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
