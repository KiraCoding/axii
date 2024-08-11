trait Module {
    fn base(&self) -> *const ();

    fn len(&self) -> usize;

    fn as_slice(&self) -> &[u8];

    unsafe fn scan<T>(&self, pattern: &[u8]) -> Option<T>;

    unsafe fn rva<T>(&self, offset: usize) -> T;
}