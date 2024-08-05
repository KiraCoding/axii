use core::ffi::c_void;
use windows::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE};

pub trait FnPtr {
    // &self is the address to hook into and function is our hook that gets called/attached
    fn hook<F>(&self, function: F)
    where
        F: Fn() + 'static;
}

macro_rules! impl_fnptr {
    ($(($($args:ident),*)),*) => {
        $(
            // impl<R, $($args),*> FnPtr for unsafe extern "C" fn($($args),*) -> R {}
            // impl<R, $($args),*> FnPtr for unsafe extern "cdecl" fn($($args),*) -> R {}
            // impl<R, $($args),*> FnPtr for unsafe extern "win64" fn($($args),*) -> R {}

            impl<R, $($args),*> FnPtr for unsafe extern "fastcall" fn($($args),*) -> R {
                fn hook<F>(&self, Function: F) 
                where 
                    F: Fn($($args),*) -> R + 'static, {

                }
            }


            // impl<R, $($args),*> FnPtr for unsafe extern "thiscall" fn($($args),*) -> R {}

        )*
    };
}

impl_fnptr! {
    (),
    (A1),
    (A1, A2),
    (A1, A2, A3),
    (A1, A2, A3, A4)
}
