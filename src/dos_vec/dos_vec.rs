use core::mem::size_of;

use crate::{memory::{dos_allocator::DOS_ALLOCATOR, memory_chunk::MemChunk}, io::debug, panic::panic_exit};

pub const GROW_COUNT: usize = 10;

pub struct DosVec<T> {
    mem_chunk: *mut MemChunk,
    buf_ptr: *mut T,
    reserved_len: usize,
    len: usize,
}

impl<T> DosVec<T> {
    pub fn get_len(&self) -> usize {
        self.len
    }
}

impl<T> DosVec<T> {
    unsafe fn create_from_ptr(reserved_len: usize, ptr: *mut u8) -> Self {
        if ptr.is_null() {
            panic_exit("NO MEMORY FOR DOS VEC", 200);
        }
        let mem_chunk = ptr as *mut MemChunk;
        let buf_ptr = ptr.add(size_of::<MemChunk>()) as *mut T;
        DosVec {
            mem_chunk,
            buf_ptr,
            reserved_len,
            len: 0,
        }
    }

    pub fn new(reserved_len: usize) -> Self {
        unsafe {
            let size = reserved_len * size_of::<T>();
            let ptr = DOS_ALLOCATOR.alloc(size);
            DosVec::create_from_ptr(reserved_len, ptr)
        }
    }

    pub fn push(&mut self, value: T) {
        unsafe {
            if self.len >= self.reserved_len {
                let size_of_T = size_of::<T>();
                let new_reserved_len = self.reserved_len + GROW_COUNT;
                
                let new_ptr = DOS_ALLOCATOR.realloc(
                    self.mem_chunk as *mut u8,
                    self.reserved_len * size_of_T,
                    new_reserved_len * size_of_T,
                );
                if new_ptr.is_null() {
                    panic_exit("NO MEMORY FOR DOS VEC", 200);
                }

                self.mem_chunk = new_ptr as *mut MemChunk;
                self.buf_ptr = new_ptr.add(size_of::<MemChunk>()) as *mut T;
                self.reserved_len = new_reserved_len;
            }

            *self.buf_ptr.add(self.len) = value;
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<&T> {
        unsafe {
            if self.len == 0 {
                return None;
            }
            let value = &*self.buf_ptr.add(self.len - 1);
            self.len -= 1;
            Some(value)
        }
    }

    pub fn get_first(&self) -> Option<&T> {
        unsafe {
            if self.len == 0 {
                return None;
            }
            let value = &*self.buf_ptr;
            Some(value)
        }
    }
}
