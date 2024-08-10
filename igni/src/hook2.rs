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
    let ptr_bytes = ptr.as_u8_ptr();
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

    // I'm assuming this is sound... but it'll likely crash due to W^X

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
}

pub trait Hook<F>: Copy {
    fn as_u8_ptr(self) -> *mut u8;
    fn trampoline(f: F) -> Closure<F>;
}

macro_rules! impl_hook {
    ($($a:ident: $at:ident),*) => {
        impl<F, R, $($at),*> Hook<F> for unsafe extern "C" fn($($at),*) -> R
        where
            F: FnMut($($at),*) + 'static
        {
            fn as_u8_ptr(self) -> *mut u8 {
                self as *mut u8
            }

            fn trampoline(f: F) -> Closure<F> {
                unsafe extern "C" fn thunk<F,R, $($at),*>( $($a: $at),* )
                where
                    F: FnMut($($at),* )
                {
                    let p = STATIC_CONTEXT.swap(null_mut(), Ordering::Relaxed) as *mut ClosureInner<F>;
                    ((*p).data)( $($a),* );
                }
                Closure {
                    inner: Box::new(ClosureInner {
                        ptr: thunk::<F, R, $($at),*> as *const (),
                        data: f,
                    })
                }
            }
        }
    };
}

impl_hook!();
impl_hook!(a: A);
