use igni::program::program;
use std::sync::LazyLock;

static RTTI_SYSTEM_TABLE: LazyLock<RTTISystemTable> = LazyLock::new(|| RTTISystemTable {
    get: unsafe { program().rva(0x285D60) },
    register_global_function: unsafe { program().rva(0x146A5f0) },
});

#[derive(Debug)]
struct RTTISystemTable {
    get: unsafe extern "C" fn() -> *mut RTTISystem,
    register_global_function: unsafe extern "C" fn(*mut RTTISystem, *mut CFunction),
}

#[derive(Debug, Clone, Copy)]
pub struct RTTISystem {}

impl RTTISystem {
    pub fn get() -> Self {
        unsafe { *(RTTI_SYSTEM_TABLE.get)() }
    }

    pub fn register_global_function(&self, function: *mut CFunction) {
        unsafe {
            (RTTI_SYSTEM_TABLE.register_global_function)(self as *const _ as *mut _, function)
        }
    }
}

pub struct CFunction {}
