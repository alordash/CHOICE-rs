use core::arch::asm;
use core::ops::{Deref, DerefMut};

use crate::dos_vec::dos_vec::DosVec;
use crate::io::{debug, newline, print_str};

#[derive(Clone, PartialEq, Eq)]
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
    pub fn empty() -> Self {
        String::instantiate(0)
    }

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
                in("ah") 0x40_u8,
                in("bx") 0x01,
                in("cx") self.vec.get_len(),
                in("dx") self.vec.buf_ptr as u16,
            )
        }
    }

    pub fn split(&self, spliterator: impl Fn(u8) -> bool) -> DosVec<String> {
        let mut strs = DosVec::<String>::new(1);
        let mut start = 0;
        for i in 0..self.get_len() {
            let c = self[i];
            if spliterator(c) {
                unsafe {
                    strs.push(String::from_raw_parts(
                        self.buf_ptr.add(start),
                        i + 1 - start,
                    ))
                };
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

    pub fn truncate(&mut self, truncator: impl Fn(u8) -> bool) {
        let len = self.get_len();
        let mut start = 0;
        let mut end = len;
        for i in 0..len {
            let c = self[i];
            if truncator(c) {
                start += 1;
            } else {
                break;
            }
        }

        let mut i = len;
        while i > start {
            i -= 1;
            let c = self[i];
            if truncator(c) {
                end = i;
                break;
            }
        }

        if end > start {
            unsafe {
                *self = String::from_raw_parts(self.buf_ptr.add(start), end - start);
            }
        } else {
            unsafe {
                *self = String::instantiate(0);
            }
        }
    }

    pub fn try_to_i32(&self) -> Option<i32> {
        fn is_char_num(c: u8) -> bool {
            ('0' as u8..'9' as u8).contains(&c)
        }
        let mut num = 0i32;
        let mut sign = 1i32;
        let mut begin = 0;
        for i in 0..self.get_len() {
            let c = self[i];
            if c == '-' as u8 {
                sign *= -1;
                begin = i + 1;
            } else {
                break;
            }
        }

        for i in begin..self.get_len() {
            let c = self[i];
            if !is_char_num(c) {
                return None;
            }
            num = num * 10 + (c - '0' as u8) as i32;
        }
        return Some(num * sign);
    }

    pub fn substring(&self, begin: usize, end: usize) -> String {
        let len = end as isize - begin as isize;
        if len <= 0 {
            return String::empty();
        }
        unsafe { String::from_raw_parts(self.buf_ptr.add(begin), len as usize) }
    }
}
