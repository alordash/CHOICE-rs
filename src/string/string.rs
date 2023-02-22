use core::arch::asm;
use core::ops::Deref;

use crate::dos_vec::dos_vec::DosVec;
use crate::io::{print_char, print_str};

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

impl String {
    pub fn from_str(s: &str) -> Self {
        let mut string = String::instantiate(s.len());
        for (i, ch) in s.as_bytes().iter().enumerate() {
            string.vec[i] = *ch;
        }
        return string;
    }

    pub unsafe fn from_raw_parts(begin: *const u8, len: usize) -> Self {
        String {
            vec: DosVec::from_raw_parts(begin, len)
        }
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
}
