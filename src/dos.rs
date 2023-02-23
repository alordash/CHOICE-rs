use core::arch::asm;

use crate::io::{debug, newline, print_char, print_num, print_str, println, println_bool};

pub fn wait(millis: u32) {
    unsafe {
        let real_millis = millis * 1000;
        let cx = ((real_millis & 0xFFFF0000) >> 16) as u16;
        let dx = (real_millis & 0x0000FFFF) as u16;
        asm!(
            "int 15h",
            in("ax") 0x8600 as u16,
            in("cx") cx,
            in("dx") dx,
        );
    }
}

pub fn set_wait_interval(millis: u32, byte_ptr: *mut u8) {
    unsafe {
        *byte_ptr = 0;
        let real_millis = millis * 1000;
        let cx = ((real_millis & 0xFFFF0000) >> 16) as u16;
        let dx = (real_millis & 0x0000FFFF) as u16;
        asm!(
            "int 15h",
            in("ax") 0x8300 as u16,
            in("cx") cx,
            in("dx") dx,
            in("bx") byte_ptr as u16,
        );
        // loop {
        //     asm!("nop");
        //     if *byte_ptr > 0 {
        //         *byte_ptr = 0;
        //         break;
        //     }
        // }
    }
}

pub fn is_stdin_has_chars() -> bool {
    unsafe {
        let al: u8;
        asm!(
            "int 21h",
            in("ah") 0x0B_u8,
            out("al") al
        );
        return al == 0xFF_u8;
    }
}

pub fn try_get_char() -> Option<u16> {
    unsafe {
        let al: u8;
        let succesfully_read: u8;
        asm!(
            "int 21h",
            "mov bl, 1",
            "jz 1f",
            "jnz 2f",
            "1:",
            "mov bl, 0",
            "2:",
            in("ah") 0x06_u8,
            in("dl") 0xFF_u8,
            out("al") al,
            out("bl") succesfully_read
        );
        
        if al != 0 {
            Some(al as u16)
        } else {
            None
        }
    }
}
