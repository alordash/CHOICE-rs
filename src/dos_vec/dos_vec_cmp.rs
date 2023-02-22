use super::dos_vec::DosVec;

impl<T: PartialEq> PartialEq for DosVec<T> {
    fn eq(&self, other: &Self) -> bool {
        if other.get_len() != self.get_len() {
            return false;
        }
        for i in 0..self.get_len() {
            if other[i] != self[i] {
                return false;
            }
        }
        return true;
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<T: Eq> Eq for DosVec<T> {}
