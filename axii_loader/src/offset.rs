use core::ffi::{c_char, c_size_t, c_void, CStr};
use core::ptr::null;
use igni::program::program;
use serde::Deserialize;
use serde_json::from_str;
use std::{collections::HashMap, env::current_dir, fs::read_to_string, sync::LazyLock};

static OFFSETS: LazyLock<Offsets> = LazyLock::new(Offsets::init);

#[export_name = "resolve"]
pub unsafe extern "C-unwind" fn resolve_cpp(symbol: *const c_char) -> *const c_void {
    if symbol.is_null() {
        return null();
    }

    let c_str = unsafe { CStr::from_ptr(symbol) };

    match c_str.to_str() {
        Ok(str) => resolve(str).cast(),
        Err(_) => null(),
    }
}

pub(crate) fn resolve(symbol: &str) -> *const u8 {
    let offset = OFFSETS.inner.get(symbol).copied().unwrap();
    unsafe { program().text().rva(offset) }
}

struct Offsets {
    inner: HashMap<String, usize>,
}

impl Offsets {
    fn init() -> Self {
        let path = current_dir().unwrap().join("witcher3map.json");
        let json_map = read_to_string(path).unwrap();
        let parsed_json: Map = from_str(&json_map).unwrap();

        let inner = parsed_json
            .addresses
            .into_iter()
            .map(|address| {
                let offset_str = address.offset.split(':').last().unwrap();
                let offset = usize::from_str_radix(offset_str, 16).unwrap();
                (address.symbol, offset)
            })
            .collect();

        Self { inner }
    }
}

#[derive(Deserialize)]
struct Map {
    #[serde(rename = "Addresses")]
    addresses: Vec<Address>,
}

#[derive(Deserialize)]
struct Address {
    symbol: String,
    offset: String,
}
