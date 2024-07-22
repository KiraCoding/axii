#[cfg(not(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")))]
compile_error!("This crate can only be compiled for the x86_64-pc-windows-msvc target");

use std::env::current_dir;
use std::fs::read_dir;
use std::io::{stderr, stdout};
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::AsRawHandle;
use std::path::PathBuf;
use tracing::error;
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

    paths.iter().for_each(|path| {
        let w_path: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();

        match LoadLibraryW(PCWSTR(w_path.as_ptr())) {
            Ok(module) => init_plugin(module),
            Err(err) => error!("{}", err),
        };
    });
}

fn init_plugin(module: HMODULE) {
    unsafe { GetProcAddress(module, s!("plugin")).unwrap()() };
}

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
