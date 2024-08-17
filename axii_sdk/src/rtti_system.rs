use crate::{function::Function, resolve};
use igni::program::program;
use std::sync::LazyLock;

static RTTI_SYSTEM_TABLE: LazyLock<RTTISystemTable> = LazyLock::new(RTTISystemTable::init);

#[derive(Debug)]
struct RTTISystemTable {
    get: unsafe extern "C" fn() -> *mut RTTISystem,
    register_global_function: unsafe extern "C" fn(*mut RTTISystem, *mut Function),
}

impl RTTISystemTable {
    fn init() -> Self {
        #[rustfmt::skip]
        let pattern = &[
            0x48, 0x83, 0xEC, 0x28,                   // SUB RSP, 0x28
            0x48, 0x8B, 0x05, 0x2D, 0x19, 0x81, 0x05, // MOV RAX, qword ptr [null_0000000000000000h_145a97698]
            0x48, 0x85, 0xC0,                         // TEST RAX, RAX
            0x0F, 0x85, 0x26,                         // JNZ LAB_140285e9a
            
        ];

        Self {
            get: dbg!(unsafe { program().text().scan(pattern).unwrap() }),
            register_global_function: dbg!(resolve("CRTTISystem::RegisterGlobalFunction")),
        }
    }
}

pub struct RTTISystem;

impl RTTISystem {
    fn get() -> *mut Self {
        unsafe { (RTTI_SYSTEM_TABLE.get)() }
    }

    // pub fn register_global_function(function: Function) {
    //     unsafe { (RTTI_SYSTEM_TABLE.register_global_function)(Self::get(), function.this) }
    // }
}

