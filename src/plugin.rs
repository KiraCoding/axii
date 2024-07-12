use windows::Win32::Foundation::HMODULE;

pub(crate) struct Plugin {
    module: HMODULE,
}

impl Plugin {
    fn get_module(&self) -> HMODULE {
        self.module
    }
}

impl From<HMODULE> for Plugin {
    fn from(module: HMODULE) -> Self {
        Self { module }
    }
}
