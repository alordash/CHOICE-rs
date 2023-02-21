pub struct MemChunk {
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

    pub fn new(len: usize, start: *mut u8) -> MemChunk {
        MemChunk { len, start }
    }
}
