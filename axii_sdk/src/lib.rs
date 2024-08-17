mod function;
pub mod names_pool;
mod rtti_system;

use igni::program::program;
use serde::Deserialize;
use serde_json::from_str;
use std::{collections::HashMap, env::current_dir, fs::read_to_string, sync::LazyLock};

pub use function::Function;
pub use rtti_system::RTTISystem;

static OFFSETS: LazyLock<Offsets> = LazyLock::new(Offsets::init);

pub(crate) fn resolve<T>(symbol: &str) -> T {
    let offset = OFFSETS.inner.get(symbol).copied().unwrap();
    unsafe { program().text().rva(offset) }
}

struct Offsets {
    inner: HashMap<String, usize>,
}

impl Offsets {
    fn init() -> Self {
        dbg!(current_dir().unwrap());

        let path = current_dir()
            .unwrap()
            .join("x64_dx12")
            .join("witcher3map.json");
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
