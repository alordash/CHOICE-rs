use core::ops::{Index, IndexMut};

use super::dos_vec::DosVec;

impl<T> Index<usize> for DosVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.buf_ptr.add(index) }
    }
}

impl<T> IndexMut<usize> for DosVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *self.buf_ptr.add(index) }
    }
}
