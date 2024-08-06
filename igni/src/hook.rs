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
                    todo!()
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

