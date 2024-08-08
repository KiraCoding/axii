use core::ptr::{addr_of, copy_nonoverlapping};
use windows::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE};

pub unsafe fn copy_rw<T>(src: *const T, dst: *mut T, count: usize) {
    let size = count * size_of::<T>();
    let mut old_protect = Default::default();
    unsafe { VirtualProtect(src.cast(), size, PAGE_EXECUTE_READWRITE, &mut old_protect).unwrap() };
    unsafe { copy_nonoverlapping(src, dst, count) };
    unsafe { VirtualProtect(src.cast(), size, old_protect, &mut old_protect).unwrap() };
}

pub trait Hookable<F>: Copy {
    fn hook(self, function: F) {
        let ptr = self.as_u8_ptr();
        println!("Ptr: {:p}", ptr);

        let function = addr_of!(function) as usize;
        println!("Closure: {:x}", function);

        let bytes = {
            let mut jmp_bytes: [u8; 14] = [
                0xFF, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ];
            jmp_bytes[6..14].copy_from_slice(&function.to_le_bytes());
            jmp_bytes
        };
        println!("Bytes: {:x?}", bytes);

        unsafe { copy_rw(bytes.as_ptr(), ptr, bytes.len()) };
    }

    fn as_u8_ptr(self) -> *mut u8;
}

macro_rules! impl_hookable {
    ($(($($args:ident),*)),*) => {
        $(
            impl<F, R, $($args),*> Hookable<F> for *const unsafe extern "C" fn($($args),*) -> R
            where
                F: FnMut($($args),*) {
                    fn as_u8_ptr(self) -> *mut u8 {
                        self as *mut u8
                    }
                }

            impl<F, R, $($args),*> Hookable<F> for *const unsafe extern "cdecl" fn($($args),*) -> R
            where
                F: FnMut($($args),*) {
                    fn as_u8_ptr(self) -> *mut u8 {
                        self as *mut u8
                    }
                }

            impl<F, R, $($args),*> Hookable<F> for *const unsafe extern "win64" fn($($args),*) -> R
            where
                F: FnMut($($args),*) {
                    fn as_u8_ptr(self) -> *mut u8 {
                        self as *mut u8
                    }
                }

            impl<F, R, $($args),*> Hookable<F> for *const unsafe extern "fastcall" fn($($args),*) -> R
            where
                F: FnMut($($args),*) {
                    fn as_u8_ptr(self) -> *mut u8 {
                        self as *mut u8
                    }
                }

            impl<F, R, $($args),*> Hookable<F> for *const unsafe extern "thiscall" fn($($args),*) -> R
            where
                F: FnMut($($args),*) {
                    fn as_u8_ptr(self) -> *mut u8 {
                        self as *mut u8
                    }
                }
        )*
    };
}

impl_hookable! {
    (),
    (A1),
    (A1, A2),
    (A1, A2, A3),
    (A1, A2, A3, A4)
}
