#[cfg(not(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")))]
compile_error!("This crate can only be compiled for the x86_64-pc-windows-msvc target");

mod steam;

use clap::{Parser, ValueEnum};
use std::fs::{copy, create_dir_all};
use std::path::Path;
use std::process::Command;
use steam::Library;
use windows_registry::Result;

const WITCHER3_APPID: &str = "292030";
const AXII_LOADER: &str = env!("CARGO_CDYLIB_FILE_AXII_LOADER_AXII");
const AXII_PROXY: &str = env!("CARGO_CDYLIB_FILE_AXII_PROXY_DINPUT8");

#[derive(Parser)]
struct Cli {
    #[arg(value_enum, default_value_t = Launcher::Steam)]
    launcher: Launcher,

    #[arg(value_enum, default_value_t = DirectX::Dx12)]
    directx: DirectX,
}

#[derive(Debug, ValueEnum, Clone, Copy)]
enum Launcher {
    Steam,
    Gog,
}

#[derive(Debug, ValueEnum, Clone, Copy)]
enum DirectX {
    Dx11,
    Dx12,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.launcher {
        Launcher::Steam => (),
        Launcher::Gog => unimplemented!("The gog backend is not yet implemented"),
    }

    let path = Library::get()
        .install_dir(WITCHER3_APPID)
        .unwrap()
        .join("bin");

    let whse = Path::new(&path).join("whse");
    let x64_dx12 = Path::new(&path).join("x64_dx12");

    create_dir_all(&whse).unwrap();

    copy(AXII_LOADER, whse.join("axii.dll")).unwrap();
    copy(AXII_PROXY, x64_dx12.join("dinput8.dll")).unwrap();

    Command::new(&x64_dx12.join("witcher3.exe"))
        .current_dir(x64_dx12)
        .spawn()
        .unwrap();

    Ok(())
}
