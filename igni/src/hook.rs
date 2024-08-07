use core::ptr::{addr_of, copy_nonoverlapping};
use std::ptr::null;
use windows::Win32::System::Memory::{
    VirtualAlloc, VirtualProtect, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READWRITE,
};

pub trait Hookable<F> {
    fn hook(&self, function: F);
}

macro_rules! impl_hookable {
    ($(($($args:ident),*)),*) => {
        $(
            impl<F, R, $($args),*> Hookable<F> for unsafe extern "C" fn($($args),*) -> R
            where
                F: FnMut($($args),*)
            {
                fn hook(&self, function: F) {
                    todo!()
                }
            }

            impl<F, R, $($args),*> Hookable<F> for unsafe extern "cdecl" fn($($args),*) -> R
            where
                F: FnMut($($args),*)
            {
                fn hook(&self, function: F) {
                    todo!()
                }
            }

            impl<F, R, $($args),*> Hookable<F> for unsafe extern "win64" fn($($args),*) -> R
            where
                F: FnMut($($args),*)
            {
                fn hook(&self, function: F) {
                    todo!()
                }
            }

            impl<F, R, $($args),*> Hookable<F> for unsafe extern "fastcall" fn($($args),*) -> R
            where
                F: FnMut($($args),*)
            {
                fn hook(&self, function: F) {
                    todo!()
                }
            }

            impl<F, R, $($args),*> Hookable<F> for unsafe extern "thiscall" fn($($args),*) -> R
            where
                F: FnMut($($args),*)
            {
                fn hook(&self, function: F) {
                    let ptr = addr_of!(self).cast();
                    let function = addr_of!(function).cast();

                    hook(ptr, function)
                }
            }
        )*
    };
}

fn alloc_page_near_addr(ptr: *const u8) -> *const u8 {
    let addr = null();

    let out_addr = unsafe {
        VirtualAlloc(
            Some(addr),
            0,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };

    out_addr.cast()
}

fn write_abs_jmp(mem: *const u8, function: *const usize) {
    let mut abs_jmp = [
        0x49, 0xBA, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0xFF, 0xE2,
    ];
    abs_jmp[2..10].copy_from_slice(&(function as u64).to_le_bytes());
    unsafe { copy_rw(mem, abs_jmp.as_mut_ptr(), abs_jmp.len()) };
}

fn hook(ptr: *const u8, function: *const usize) {
    let mut addr_to_jump = alloc_page_near_addr(ptr);
    write_abs_jmp(addr_to_jump, function);

    //32 bit relative jump opcode is E9, takes 1 32 bit operand for jump offset
    let jmp_inst = [0xE9, 0x0, 0x0, 0x0, 0x0];
    unsafe {
        copy_nonoverlapping(
            &mut addr_to_jump,
            jmp_inst.as_mut_ptr().add(1),
            size_of::<u32>(),
        )
    };

    //install the hook
    unsafe { copy_rw(jmp_inst.as_ptr(), ptr.cast_mut(), jmp_inst.len()) };
}

pub unsafe fn copy_rw<T>(src: *const T, dst: *mut T, count: usize) {
    let size = count * size_of::<T>();
    let mut old_protect = Default::default();
    unsafe { VirtualProtect(src.cast(), size, PAGE_EXECUTE_READWRITE, &mut old_protect).unwrap() };
    unsafe { copy_nonoverlapping(src, dst, count) };
    unsafe { VirtualProtect(src.cast(), size, old_protect, &mut old_protect).unwrap() };
}

impl_hookable! {
    (),
    (A1),
    (A1, A2),
    (A1, A2, A3),
    (A1, A2, A3, A4)
}
