use super::dos_vec::DosVec;

impl<T: Clone> Clone for DosVec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = DosVec::<T>::new(self.reserved_len);
        new_vec.len = self.len;
        for i in 0..self.len {
            unsafe { *new_vec.buf_ptr.add(i) = (*self.buf_ptr.add(i)).clone() };
        }
        new_vec
    }
}
