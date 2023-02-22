use core::{mem::size_of, ptr::null_mut};

use crate::{
    memory::{dos_allocator::DOS_ALLOCATOR, memory_chunk::MemChunk},
    panic::panic_exit,
};

pub const GROW_COUNT: usize = 10;

pub struct DosVec<T> {
    pub mem_chunk: *mut MemChunk,
    pub buf_ptr: *mut T,
    pub reserved_len: usize,
    pub len: usize,
}

impl<T> DosVec<T> {
    unsafe fn create_from_ptr(reserved_len: usize, ptr: *mut u8) -> Self {
        if reserved_len > 0 && ptr.is_null() {
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

    unsafe fn grow(&mut self, extra_size: usize) {
        let new_reserved_len = self.reserved_len + extra_size;
        let size_of_T = size_of::<T>();

        let new_ptr = if self.reserved_len > 0 {
            DOS_ALLOCATOR.realloc(
                self.mem_chunk as *mut u8,
                self.reserved_len * size_of_T,
                new_reserved_len * size_of_T,
            )
        } else {
            DOS_ALLOCATOR.alloc(new_reserved_len * size_of_T)
        };
        if new_ptr.is_null() {
            panic_exit("NO MEMORY FOR GROWING DOS VEC", 200);
        }

        self.mem_chunk = new_ptr as *mut MemChunk;
        self.buf_ptr = new_ptr.add(size_of::<MemChunk>()) as *mut T;
        self.reserved_len = new_reserved_len;
    }
}

impl<T: Copy> DosVec<T> {
    pub fn new(reserved_len: usize) -> Self {
        unsafe {
            let size = reserved_len * size_of::<T>();
            let ptr = DOS_ALLOCATOR.alloc(size);
            DosVec::create_from_ptr(reserved_len, ptr)
        }
    }

    pub fn from_raw_parts(begin: *const T, len: usize) -> Self {
        unsafe {
            let mut dos_vec = DosVec::new(len);
            for i in 0..len {
                dos_vec[i] = *begin.add(i);
            }
            dos_vec.len = len;
            dos_vec
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            DOS_ALLOCATOR.dealloc(
                self.mem_chunk as *mut u8,
                self.reserved_len * size_of::<T>(),
            );
            self.mem_chunk = null_mut();
            self.buf_ptr = null_mut();
            self.reserved_len = 0;
            self.len = 0;
        }
    }

    pub fn push(&mut self, value: T) {
        unsafe {
            if self.len >= self.reserved_len {
                self.grow(GROW_COUNT);
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

    pub fn get_len(&self) -> usize {
        self.len
    }
}
