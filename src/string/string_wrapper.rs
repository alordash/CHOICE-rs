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
            "mov bx, 01h",
            // "mov dx, {0}",
            // "mov cx, {1}",
            "int 21h",
            in("dx") self.ptr as u16,
            in("cx") self.len
        )
    }

    pub unsafe fn print_v2(&self) {
        for offset in 0..1 {
            let char = *self.ptr.offset(1);
            print_char(char as char);
        }
        // print_char(*self.ptr.offset(1) as char)
    }
}
