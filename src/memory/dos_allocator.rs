use core::cell::UnsafeCell;
use core::mem::size_of;
use core::ptr::null_mut;

use crate::{io::{debug, print_num, print_str, println}, panic::panic_exit};

use super::memory_chunk::MemChunk;

const ALLOCATED_MEM_SIZE_BYTES: usize = 1000;

pub struct DosAllocator<const MEM_SIZE_BYTES: usize> {
    pub memory: [u8; MEM_SIZE_BYTES],
}

impl<const MEM_SIZE_BYTES: usize> DosAllocator<MEM_SIZE_BYTES> {
    pub const fn alloc_memory() -> DosAllocator<MEM_SIZE_BYTES> {
        unsafe {
            let mut memory = [0x00; MEM_SIZE_BYTES];
            DosAllocator { memory }
        }
    }

    pub fn zero_memory(&mut self) {
        for i in 0..MEM_SIZE_BYTES {
            self.memory[i] = 0;
        }
    }
}

unsafe impl<const MEM_SIZE_BYTES: usize> Sync for DosAllocator<MEM_SIZE_BYTES> {}

pub static mut DOS_ALLOCATOR: DosAllocator<ALLOCATED_MEM_SIZE_BYTES> =
    DosAllocator::<ALLOCATED_MEM_SIZE_BYTES>::alloc_memory();

impl<const MEM_SIZE_BYTES: usize> DosAllocator<MEM_SIZE_BYTES> {
    pub unsafe fn alloc(&mut self, size: usize) -> *mut u8 {
        if size == 0 {
            return null_mut();
        }
        let mem_begin_ptr = self.memory.as_mut_ptr();
        let mut mem_ptr = mem_begin_ptr.clone();

        println();
        let required_size = size + size_of::<MemChunk>();
        // debug("Required size: ", required_size as i32);
        debug("Begin: ", mem_begin_ptr as i32);

        let suitable_mem_ptr = loop {
            let mem_chunk = &*(mem_ptr as *const MemChunk);
            debug("Mem ptr: ", mem_ptr as i32);
            debug("Len: ", mem_chunk.get_len() as i32);
            
            if mem_chunk.get_len() == 0 {
                let left_space = MEM_SIZE_BYTES as isize - mem_ptr.offset_from(mem_begin_ptr) as isize - required_size as isize;
                debug("Found empty chunk: ", mem_ptr as i32);
                // debug("Left space: ", left_space as i32);
                if left_space >= 0 {
                    break Some(mem_ptr);
                }
                return null_mut();
            }
            mem_ptr = mem_ptr.add(mem_chunk.get_len() + size_of::<MemChunk>());
            if mem_ptr.offset_from(mem_begin_ptr) >= MEM_SIZE_BYTES as isize {
                return null_mut();
            }
        };
        if suitable_mem_ptr.is_none() {
            return null_mut();
        }
        let suitable_mem_ptr = suitable_mem_ptr.unwrap();
        *(suitable_mem_ptr as *mut MemChunk) = MemChunk::new(size, suitable_mem_ptr);

        // debug("Found suitable mem ptr: ", suitable_mem_ptr as i32);

        return suitable_mem_ptr;
    }

    pub unsafe fn dealloc(&mut self, ptr: *mut u8, size: usize) {
        let mem_chunk_ptr = ptr as *mut MemChunk;
        if (&*mem_chunk_ptr).get_len() != size {
            panic_exit("ERROR DEALLOCATING: WRONG SIZE", 100);
        }
        *mem_chunk_ptr = MemChunk::new(0, ptr);
    }

    pub unsafe fn alloc_zeroed(&mut self, size: usize) -> *mut u8 {
        let ptr = self.alloc(size).add(size_of::<MemChunk>());
        for offset in 0..size {
            *ptr.add(offset) = 0;
        }
        ptr
    }

    pub unsafe fn realloc(&mut self, ptr: *mut u8, size: usize, new_size: usize) -> *mut u8 {
        self.dealloc(ptr, size);
        let old_size = size;

        let new_ptr = self.alloc(new_size);
        for offset in 0..old_size {
            *(new_ptr.add(offset)) = *ptr.add(offset);
        }
        new_ptr
    }
}
