use core::arch::asm;

use crate::io::print_char;

pub struct StringWrapper {
    ptr: *const u8,
    len: usize,
}

impl StringWrapper {
    pub unsafe fn from_raw_parts(source_ptr: *const u8, len: usize) -> StringWrapper {
        StringWrapper {
            ptr: source_ptr,
            len,
        }
    }

    pub unsafe fn print(&self) {
        asm!(
            "mov ah, 40h",
            "mov bx, 1",
            "int 21h",
            in("cx") self.len - 1,
            in("dx") self.ptr as u32 + 1,
        )
    }

    pub unsafe fn print_v2(&self) {
        for offset in 1..self.len {
            let char = *self.ptr.add(offset);
            print_char(char as char);
        }
        // print_char(*self.ptr.offset(1) as char);
        // print_char(*self.ptr.offset(2) as char);
        // print_char(*self.ptr.offset(3) as char);
        // print_char(*self.ptr.offset(4) as char);
    }
}
