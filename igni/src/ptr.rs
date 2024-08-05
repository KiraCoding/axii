// pub struct Hook<F, Args, Ret>
// where
//     F: Fn(Args) -> Ret,
// {
//     ptr: F, // ptr to extern "C" fn
//     enabled: bool,
// }

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

// unsafe extern "C" fn(...) -> R
// unsafe extern "cdecl" fn(...) -> R
// unsafe extern "win64" fn(...) -> R
// unsafe extern "fastcall" fn(...) -> R
// unsafe extern "thiscall" fn(...) -> R
