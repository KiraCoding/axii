use core::ptr::{addr_of, copy_nonoverlapping};
use windows::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE};

#[derive(Debug)]
pub struct Hook {
    enabled: bool,
}

pub trait Hookable<F> {
    fn hook(&self, function: F) -> Hook;
}

macro_rules! impl_hookable {
    ($(($($args:ident),*)),*) => {
        $(
            impl<F, R, $($args),*> Hookable<F> for unsafe extern "C" fn($($args),*) -> R
            where
                F: FnMut($($args),*)
            {
                fn hook(&self, function: F) -> Hook {
                    todo!()
                }
            }

            impl<F, R, $($args),*> Hookable<F> for unsafe extern "cdecl" fn($($args),*) -> R
            where
                F: FnMut($($args),*)
            {
                fn hook(&self, function: F) -> Hook {
                    todo!()
                }
            }

            impl<F, R, $($args),*> Hookable<F> for unsafe extern "win64" fn($($args),*) -> R
            where
                F: FnMut($($args),*)
            {
                fn hook(&self, function: F) -> Hook {
                    todo!()
                }
            }

            impl<F, R, $($args),*> Hookable<F> for unsafe extern "fastcall" fn($($args),*) -> R
            where
                F: FnMut($($args),*)
            {
                fn hook(&self, function: F) -> Hook {
                    todo!()
                }
            }

            impl<F, R, $($args),*> Hookable<F> for unsafe extern "thiscall" fn($($args),*) -> R
            where
                F: FnMut($($args),*)
            {
                fn hook(&self, function: F) -> Hook {
                    let ptr = addr_of!(self).cast();
                    let function = addr_of!(function).cast();

                    hook(ptr, function)
                }
            }
        )*
    };
}

fn hook(ptr: *const usize, function: *const usize) -> Hook {
    // relay_func_memory = VirtualAlloc; // Allocate a page near the address to hook

    let absolute_jmp = [
        0x49, 0xBA, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //mov r10, addr
        0x41, 0xFF, 0xE2, //jmp r10
    ];

    let mut old_protect = Default::default();
    unsafe { VirtualProtect(ptr.cast(), 1024, PAGE_EXECUTE_READWRITE, &mut old_protect).unwrap() };

    let jump_instruction = [0xE9, 0x0, 0x0, 0x0, 0x0];

    // install the hook
    unsafe {
        copy_nonoverlapping(
            jump_instruction.as_ptr(),
            ptr.cast_mut(),
            jump_instruction.len(),
        )
    };

    Hook { enabled: true }
}

impl_hookable! {
    (),
    (A1),
    (A1, A2),
    (A1, A2, A3),
    (A1, A2, A3, A4)
}
