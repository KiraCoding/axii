use crate::plugin::Plugin;
use std::{fs::read_dir, os::windows::ffi::OsStrExt};
use windows::{core::PCWSTR, Win32::System::LibraryLoader::LoadLibraryW};

pub(crate) struct Registry {
    plugins: Vec<Plugin>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        let plugins_dir = "../axii/plugins/";

        let paths = read_dir(plugins_dir)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .unwrap()
                    .eq_ignore_ascii_case("dll")
            })
            .map(|entry| entry.path())
            .collect::<Vec<_>>();

        let paths = paths
            .iter()
            .map(|path_buf| {
                path_buf
                    .as_os_str()
                    .encode_wide()
                    .chain(Some(0))
                    .collect::<Vec<u16>>()
                    .as_ptr()
            })
            .collect::<Vec<*const u16>>();

        let end = paths
            .iter()
            .map(|&path| unsafe { LoadLibraryW(PCWSTR(path)).unwrap().into() });

        Self {
            plugins: end.collect(),
        }
    }
}
