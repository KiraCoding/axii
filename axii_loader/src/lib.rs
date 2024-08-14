#![feature(c_size_t)]

#[cfg(not(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")))]
compile_error!("This crate can only be compiled for the x86_64-pc-windows-msvc target");

pub mod offset;

use igni::hook::hook;
use igni::program::program;
use std::env::current_dir;
use std::fs::read_dir;
use std::io::{stderr, stdout};
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::AsRawHandle;
use std::path::PathBuf;
use tracing::{error, info};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use windows::core::{s, w, PCWSTR};
use windows::Win32::Foundation::{HANDLE, HMODULE};
use windows::Win32::System::Console::{
    AllocConsole, GetConsoleMode, SetConsoleMode, SetConsoleTitleW, SetStdHandle, CONSOLE_MODE,
    ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_ERROR_HANDLE, STD_OUTPUT_HANDLE,
};
use windows::Win32::System::Diagnostics::Debug::{SetErrorMode, SEM_FAILCRITICALERRORS};
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[no_mangle]
unsafe extern "system" fn loader() {
    SetErrorMode(SEM_FAILCRITICALERRORS);

    init_console();
    init_tracing();

    let paths = read_plugins_dir();

    if paths.is_empty() {
        info!("No plugins found");
        return;
    }

    paths.iter().for_each(|path| {
        let w_path: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();

        match LoadLibraryW(PCWSTR(w_path.as_ptr())) {
            Ok(module) => init_plugin(module),
            Err(err) => error!("{}", err),
        };
    });
}

#[inline(always)]
fn init_plugin(module: HMODULE) {
    #[rustfmt::skip]
    let pattern = &[
        0x48, 0x89, 0x5C, 0x24, 0x10,             // MOV qword ptr [RSP + local_res10], RBX
        0x57,                                     // PUSH RDI
        0x48, 0x83, 0xEC, 0x20,                   // SUB RSP, 0x20
        0xBA, 0x10, 0x00, 0x00, 0x00,             // MOV EDX, 0x10
        0xB9, 0xC0, 0x00, 0x00, 0x00,             // MOV ECX, 0xC0
        0xE8, 0x67, 0x9B, 0xDD, 0xFE,             // CALL AllocateFunc
        0x33, 0xDB,                               // XOR EBX, EBX
        0x48, 0x8B, 0xF8,                         // MOV RDI, memory
        0x48, 0x85, 0xC0,                         // TEST memory, memory
        0x74, 0x41,                               // JZ LAB_1414aab94
        0x33, 0xD2,                               // XOR EDX, EDX
        0x41, 0xB8, 0xC0, 0x00, 0x00, 0x00,       // R8D, 0xC0
        0x48, 0x8B, 0xC8,                         // MOV RCX, memory
        0xE8, 0x8D, 0x55, 0xB2, 0xFF,             // CALL FUN_140fd00f0
        0xE8, 0x38, 0x98, 0xDD, 0xFE,             // CALL CNamesPool::Get
        0x48, 0x8D, 0x15, 0xE9, 0x87, 0x01, 0x01, // LEA RDX, [u_EngineTimeFromFloat_1424c3358
    ];

    let addr = unsafe { GetProcAddress(module, s!("plugin")).unwrap() };

    let factory: unsafe extern "C" fn() = unsafe { program().text().scan(pattern).unwrap() };
    

    unsafe { addr() };
}

#[inline(always)]
fn read_plugins_dir() -> Vec<PathBuf> {
    let path = current_dir()
        .unwrap()
        .join("..\\whse\\plugins")
        .canonicalize()
        .unwrap();

    read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("dll"))
            {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}

#[inline(always)]
unsafe fn init_console() {
    AllocConsole().unwrap_unchecked();
    SetConsoleTitleW(w!("The Witcher 3: Console")).unwrap_unchecked();

    let stdout = HANDLE(stdout().as_raw_handle() as isize);
    let stderr = HANDLE(stderr().as_raw_handle() as isize);

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

#[inline(always)]
unsafe fn init_tracing() {
    let path = current_dir().unwrap().join("../whse/logs");

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::HOURLY)
        .filename_prefix("axii")
        .filename_suffix("log")
        .build(path)
        .unwrap();

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_appender)
        .with_ansi(false)
        .with_level(true)
        .with_span_events(FmtSpan::FULL);

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_writer(stdout)
        .with_ansi(true)
        .with_level(true)
        .with_span_events(FmtSpan::FULL);

    tracing_subscriber::registry()
        .with(file_layer)
        .with(stdout_layer)
        .init();
}
