use core::arch::asm;
use core::ops::{Deref, DerefMut};

use crate::dos_vec::dos_vec::DosVec;
use crate::io::{newline, print_str, debug};

#[derive(Clone)]
pub struct String {
    vec: DosVec<u8>,
}

impl String {
    fn instantiate(size: usize) -> Self {
        String {
            vec: DosVec::new(size),
        }
    }
}

impl Deref for String {
    type Target = DosVec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl DerefMut for String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

impl String {
    pub fn from_str(s: &str) -> Self {
        let len = s.len();
        let s_ptr = s.as_ptr();
        let mut string = String::instantiate(s.len());
        unsafe {
            for i in 0..len {
                string[i] = *s_ptr.add(i);
            }
        }
        string.len = len;
        
        return string;
    }

    pub unsafe fn from_raw_parts(begin: *const u8, len: usize) -> Self {
        String {
            vec: DosVec::from_raw_parts(begin, len),
        }
    }

    pub fn from_vec(vec: &DosVec<u8>) -> Self {
        let len = vec.get_len();
        let mut string = String::instantiate(len);
        for i in 0..len {
            string.vec[i] = vec[i];
        }
        string.vec.len = len;
        return string;
    }

    pub fn print(&self) {
        unsafe {
            asm!(
                "int 21h",
                in("bx") 0x01,
                in("ah") 0x40_u8,
                in("cx") self.vec.get_len(),
                in("dx") self.vec.buf_ptr as i32,
            )
        }
    }

    pub fn split(&self, spliterator: u8) -> DosVec<String> {
        let mut strs = DosVec::<String>::new(1);
        let mut start = 0;
        for i in 0..self.get_len() {
            let c = self[i];
            if c == spliterator {
                unsafe { strs.push(String::from_raw_parts(self.buf_ptr.add(start), i + 1 - start)) };
                start = i + 1;
            }
        }
        if start < self.get_len() {
            unsafe {
                strs.push(String::from_raw_parts(
                    self.buf_ptr.add(start),
                    self.get_len() - start,
                ))
            };
        }
        return strs;
    }
}