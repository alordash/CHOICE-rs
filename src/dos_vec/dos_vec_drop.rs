use core::ptr::null_mut;

use super::dos_vec::DosVec;

impl<T> Drop for DosVec<T> {
    fn drop(&mut self) {
        unsafe {
            // debug("Dropping vec, ptr: ", self.mem_chunk as i32);
            if self.mem_chunk == null_mut() {
                return;
            }
            self.clear();
        }
    }
}
