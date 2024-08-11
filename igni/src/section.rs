use core::slice::from_raw_parts;
use core::mem::transmute_copy;
use rayon::{iter::IndexedParallelIterator, slice::ParallelSlice};

#[derive(Debug)]
pub struct Section {
    pub(crate) name: String,
    pub(crate) base: *const (),
    pub(crate) len: usize,
}

impl Section {
    #[inline]
    pub fn base(&self) -> *const () {
        self.base
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { from_raw_parts(self.base.cast(), self.len) }
    }

    pub fn scan<T>(&self, pattern: &[u8]) -> Option<T> {
        self.as_slice()
            .par_windows(pattern.len())
            .position_first(|window| {
                pattern
                    .iter()
                    .enumerate()
                    .all(|(i, &p)| p == 0xFF || window[i] == p)
            })
            .map(|offset| unsafe { self.rva(offset) })
    }

    #[inline]
    pub unsafe fn rva<T>(&self, offset: usize) -> T {
        unsafe { transmute_copy(&(self.base as *const u8).add(offset)) }
    }
}

