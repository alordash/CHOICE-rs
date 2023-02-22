pub struct MemChunk {
    occupied: bool,
    len: usize,
    start: *mut u8,
}

impl MemChunk {
    pub fn get_len(&self) -> usize {
        self.len
    }

    pub fn get_start(&self) -> *mut u8 {
        self.start
    }

    pub fn get_occupied(&self) -> bool {
        self.occupied
    }

    pub fn new(occupied: bool, len: usize, start: *mut u8) -> MemChunk {
        MemChunk {
            occupied,
            len,
            start,
        }
    }
}
