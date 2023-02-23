use core::ptr::null_mut;

use super::dos_vec::DosVec;

impl<T> Drop for DosVec<T> {
    fn drop(&mut self) {
        if self.mem_chunk == null_mut() {
            return;
        }
        self.clear();
    }
}
