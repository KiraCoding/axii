use crate::section::Section;
use core::ffi::CStr;
use core::mem::{transmute_copy, zeroed};
use core::slice::from_raw_parts;
use rayon::iter::IndexedParallelIterator;
use rayon::slice::ParallelSlice;
use windows::Win32::System::Memory::VirtualAlloc;
use std::sync::LazyLock;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::Diagnostics::Debug::{IMAGE_NT_HEADERS64, IMAGE_SECTION_HEADER};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
use windows::Win32::System::SystemServices::IMAGE_DOS_HEADER;
use windows::Win32::System::Threading::GetCurrentProcess;

static PROGRAM: LazyLock<Program> = LazyLock::new(Program::init);

pub fn program() -> &'static Program {
    Program::new()
}

#[derive(Debug)]
pub struct Program {
    base: *const usize,
    len: usize,
    sections: Vec<Section>,
}

impl Program {
    #[inline]
    #[must_use]
    pub fn new() -> &'static Self {
        &PROGRAM
    }

    /// Returns a raw pointer to the programs base.
    #[inline]
    pub fn base(&self) -> *const usize {
        self.base
    }

    /// Returns the length of the program in memory.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn sections(&self) -> &[Section] {
        &self.sections
    }

    pub fn rva<T>(&self, offset: usize) -> T {
        unsafe { transmute_copy(&self.base.add(offset)) }
    }

    /// Returns a slice containing the entire program.
    pub fn as_slice(&self) -> &[u8] {
        unsafe { from_raw_parts(self.base.cast(), self.len) }
    }

    pub fn scan(&self, pattern: &[u8]) -> Option<*const usize> {
        self.as_slice()
            .par_windows(pattern.len())
            .position_first(|window| {
                pattern
                    .iter()
                    .enumerate()
                    .all(|(i, &p)| p == 0xFF || window[i] == p)
            })
            .map(|offset| unsafe { self.base.add(offset).cast() })
    }

    fn init() -> Self {
        let base = unsafe { GetModuleHandleW(PCWSTR::null()).unwrap_unchecked().0 as *const usize };

        let len = {
            let process = unsafe { GetCurrentProcess() };
            let module = HMODULE(base.cast_mut().cast());

            let mut info = unsafe { zeroed() };

            unsafe {
                GetModuleInformation(process, module, &mut info, size_of::<MODULEINFO>() as u32)
                    .unwrap_unchecked()
            };

            info.SizeOfImage as usize
        };

        let sections = {
            let dos_header = unsafe { &*(base as *const IMAGE_DOS_HEADER) };
            let nt_headers = unsafe {
                &*((base as usize + dos_header.e_lfanew as usize) as *const IMAGE_NT_HEADERS64)
            };

            let section_header_ptr =
                (base as usize + dos_header.e_lfanew as usize + size_of::<IMAGE_NT_HEADERS64>())
                    as *const IMAGE_SECTION_HEADER;

            (0..nt_headers.FileHeader.NumberOfSections)
                .map(|index| unsafe { &*section_header_ptr.add(index as usize) })
                .map(|section| {
                    let name = unsafe {
                        CStr::from_ptr(section.Name.as_ptr() as *const i8)
                            .to_string_lossy()
                            .into_owned()
                    };

                    Section {
                        name,
                        base: unsafe { base.add(section.VirtualAddress as usize) },
                        len: unsafe { section.Misc.VirtualSize as usize },
                    }
                })
                .collect()
        };

        Self {
            base,
            len,
            sections,
        }
    }
}

unsafe impl Send for Program {}
unsafe impl Sync for Program {}
