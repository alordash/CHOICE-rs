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

pub fn get_stdin_status() -> u8 {
    unsafe {
        let al: u8;
        asm!(
            "int 21h",
            in("ah") 0x0B_u8,
            out("al") al
        );
        // debug("AL: ", al as i32);
        return al;
    }
}

pub fn direct_console_input() {
    unsafe {
        // let zf: u16;
        let al: u8;
        asm!(
            "int 21h",
            // "xor bx, bx",
            // "mov bx, zf",
            in("ah") 0x06_u8,
            in("dl") 0xFF_u8,
            // out("bx") zf,
            out("al") al
        );
        // debug("zf: ", zf as i32);
        debug("al: ", al as i32);
    }
}
