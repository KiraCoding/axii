#[derive(Debug)]
pub struct Section {
    pub(crate) name: String,
    pub(crate) base: *const usize,
    pub(crate) len: usize,
}

#[derive(Debug)]
pub enum SectionKind {
    Text,
    Custom(String),
}
