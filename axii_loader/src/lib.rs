use windows::core::w;
use windows::Win32::System::Console::{
    AllocConsole, GetConsoleMode, GetStdHandle, SetConsoleMode, SetConsoleTitleW, SetStdHandle,
    CONSOLE_MODE, ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_ERROR_HANDLE, STD_OUTPUT_HANDLE,
};

#[cfg(not(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")))]
compile_error!("This crate can only be compiled for the x86_64-pc-windows-msvc target");

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[no_mangle]
unsafe extern "system" fn loader() {
    init_console()
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
