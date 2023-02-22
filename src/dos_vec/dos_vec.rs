use core::{
    mem::{size_of, ManuallyDrop},
    ptr::null_mut,
};

use crate::{
    io::{debug, newline, print_str, println},
    memory::{dos_allocator::DOS_ALLOCATOR, memory_chunk::MemChunk},
    panic::panic_exit,
};

pub const GROW_COUNT: usize = 20;

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

    pub unsafe fn grow(&mut self, extra_size: usize) {
        // println("Starting growing");
        let new_reserved_len = self.reserved_len + extra_size;
        let size_of_t = size_of::<T>();

        // debug("current reserved_len: ", self.reserved_len as i32);
        // debug("new     reserved_len: ", new_reserved_len as i32);
        let new_ptr = if self.reserved_len > 0 {
            DOS_ALLOCATOR.realloc(
                self.mem_chunk as *mut u8,
                self.reserved_len * size_of_t,
                new_reserved_len * size_of_t,
            )
        } else {
            DOS_ALLOCATOR.alloc(new_reserved_len * size_of_t)
        };
        if new_ptr.is_null() {
            panic_exit("NO MEMORY FOR GROWING DOS VEC", 200);
        }

        // debug("Reallocated, new ptr: ", new_ptr as i32);
        // debug("old ptr:              ", self.mem_chunk as i32);
        self.mem_chunk = new_ptr as *mut MemChunk;
        self.buf_ptr = new_ptr.add(size_of::<MemChunk>()) as *mut T;
        self.reserved_len = new_reserved_len;
        // println("Done growing");
    }
}

impl<T> DosVec<T> {
    pub fn new(reserved_len: usize) -> Self {
        unsafe {
            let size = reserved_len * size_of::<T>();
            let ptr = DOS_ALLOCATOR.alloc(size);
            DosVec::create_from_ptr(reserved_len, ptr)
        }
    }

    pub fn clear(&mut self) {
        // debug("Clearing vec, len: ", self.len as i32);
        if self.len <= 0 {
            return;
        }
        unsafe {
            DOS_ALLOCATOR.dealloc(self.mem_chunk as *mut u8, (&*self.mem_chunk).get_len());
            self.mem_chunk = null_mut();
            self.buf_ptr = null_mut();
            self.reserved_len = 0;
            self.len = 0;
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

    pub fn push(&mut self, value: T) {
        unsafe {
            if self.len >= self.reserved_len {
                // println("Not enough space, growing");
                self.grow(GROW_COUNT);
            }

            *self.buf_ptr.add(self.len) = value;
            self.len += 1;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.get_len() == 0
    }

    pub fn find_idx(&self, filter: impl Fn(&T) -> bool) -> Option<usize> {
        for i in 0..self.get_len() {
            if filter(&self[i]) {
                return Some(i);
            }
        }
        None
    }
}

impl<T: Copy> DosVec<T> {
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
}

impl<T: Clone> DosVec<T> {
    pub fn remove_at(&mut self, remove_idx: usize) {
        if remove_idx >= self.len {
            return;
        }
        for i in remove_idx..self.get_len() {
            self[i] = self[i + 1].clone();
        }
        self.len -= 1;
    }

    pub fn remain_filtered(&mut self, filter: impl Fn(&T) -> bool) {
        let mut i = 0;
        let mut len = self.get_len();
        while i < len {
            let v = &self[i];
            if filter(v) {
                i += 1;
            } else {
                self.remove_at(i);
                len -= 1;
            }
        }
    }
}
