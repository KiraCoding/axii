use core::slice::from_raw_parts;
use rayon::{iter::IndexedParallelIterator, slice::ParallelSlice};

#[derive(Debug)]
pub struct Section {
    pub name: String,
    pub base: *const (),
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

    pub fn scan(&self, pattern: &[u8]) -> Option<*const u8> {
        self.as_slice()
            .par_windows(pattern.len())
            .position_first(|window| {
                pattern
                    .iter()
                    .enumerate()
                    .all(|(i, &p)| p == 0xFF || window[i] == p)
            })
            .map(|offset| unsafe { (self.base as *const u8).add(offset) })
    }
}

