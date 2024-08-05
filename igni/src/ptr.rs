pub struct Hook<F> {
    func: F,
    enabled: bool,
}

// impl Hookable for any extern fn
pub trait Hookable {}

fn main() {
    extern "C" fn add(x: u8, y: u8) -> u8 {
        x + y
    }

    extern "C" fn add3(x: u8, y: u8, z: u8) -> u8 {
        x + y + z
    }

    // hook takes closure of `fn` sig except ret
    let handle = add.hook(|x, y| x - y);
    let handle1 = add3.hook(|x, y, z| x - y);
}
