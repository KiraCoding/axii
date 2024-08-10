use core::arch::asm;
use core::ptr::{null_mut, write};
use core::sync::atomic::{AtomicPtr, Ordering};
use windows::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE};

static STATIC_CONTEXT: AtomicPtr<()> = AtomicPtr::new(null_mut());

pub struct Closure<F> {
    inner: Box<ClosureInner<F>>,
}

#[repr(C)]
struct ClosureInner<F> {
    ptr: *const (),
    data: F,
}

pub fn hook<H: Hook<F>, F>(ptr: H, f: F) {
    let ptr_bytes = dbg!(ptr.as_u8_ptr());
    let trampoline = H::trampoline(f);
    let context = Box::into_raw(trampoline.inner) as usize;

    let mut bytes = [
        // Set RAX to context - https://www.felixcloutier.com/x86/mov
        // - REX.W MovRI AX, ...
        0x48, 0xB8, 0x00, 0, 0, 0, 0, 0, 0, 0, 0,
        // Jump to stub - https://www.felixcloutier.com/x86/jmp
        0xE9, 0, 0, 0, 0, // Jump with offset
    ];

    bytes[3..][..8].copy_from_slice(&context.to_ne_bytes());

    let offset = {
        let base = ptr_bytes as usize + 11 + 4;
        let ofs = (stub as usize).wrapping_sub(base);
        ofs as u32
    };

    bytes[11..][..4].copy_from_slice(&offset.to_ne_bytes());

    dbg!(bytes);

    let mut old_protect = Default::default();
    unsafe {
        VirtualProtect(
            ptr_bytes.cast(),
            bytes.len(),
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        )
        .unwrap();

        write(ptr_bytes as *mut _, bytes);

        VirtualProtect(ptr_bytes.cast(), bytes.len(), old_protect, &mut old_protect).unwrap();
    };

    #[naked]
    unsafe extern "C" fn stub() {
        asm!(
            "push rax", // Get context var from stack, and store in a free register
            "2:",
            "cmpxchg [rip + {}], rax",
            "test rax, rax",
            "mov rax, [rsp]", // Grab the context again, since cmpxchg clobbered it
            "jnz 2b",
            "pop rax",
            "jmp [rax]",
            sym STATIC_CONTEXT,
            options(noreturn)
        );
    }

    println!("DONE")
}

pub struct HookGuard {}

pub trait Hook<F>: Copy {
    fn as_u8_ptr(self) -> *mut u8;
    fn trampoline(f: F) -> Closure<F>;
}

macro_rules! test {
    ($arg:ident, $($args:ident),* $(,)?) => {
        test!(@impl $arg, $($args),*);
        test!($($args,)*);
    };
    (@impl $($args:ident),* $(,)?) => {
        #[allow(non_snake_case)]
        impl<F, R, $($args),*> Hook<F> for unsafe extern "C" fn($($args,)*) -> R
        where
            F: FnMut($($args),*) + 'static
        {
            fn as_u8_ptr(self) -> *mut u8 {
                self as *mut u8
            }

            fn trampoline(f: F) -> Closure<F> {
                unsafe extern "C" fn thunk<F,R, $($args),*>($($args: $args),*)
                where
                    F: FnMut($($args),* )
                {
                    let p = STATIC_CONTEXT.swap(null_mut(), Ordering::Relaxed) as *mut ClosureInner<F>;
                    ((*p).data)($($args),*);
                }
                Closure {
                    inner: Box::new(ClosureInner {
                        ptr: thunk::<F, R, $($args),*> as *const (),
                        data: f,
                    })
                }
            }
        }
        
        // impl<F, R, $($args),*> Hook<F> for unsafe extern "win64" fn($($args,)*) -> R {}
    };
    () => {
        test!(@impl);
    };
}

test!(A, B, C, D);