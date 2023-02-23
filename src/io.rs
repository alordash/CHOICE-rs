use core::arch::asm;

use crate::{string::string::String, dos::{set_wait_interval, stop_wait_interval}};

pub unsafe fn get_args_len() -> u8 {
    let args_count: u8;
    asm!(
        "xor cx, cx",
        "mov cl, byte ptr ds:[80h]",
        out("cl") args_count
    );
    args_count
}

pub unsafe fn get_args_str(args_count: u8) -> String {
    let argv_ptr: u8;
    asm!(
        "xor cx, cx",
        "lea cx, byte ptr ds:[81h]",
        out("cl") argv_ptr
    );
    String::from_raw_parts(argv_ptr as *const u8, args_count as usize)
}

pub fn print_char(c: u8) {
    unsafe {
        asm!(
            "int 21h",
            in("ah") 2_u8,
            in("dl") c
        );
    }
}

pub fn print_num(mut num: i16) {
    if num == 0 {
        print_char('0' as u8);
        return;
    }
    if num.is_negative() {
        print_char('-' as u8);
        num = -num;
    }
    let mut len = 0;
    while num > 0 {
        let rem = num % 10;
        unsafe { asm!("push {}", in(reg) rem) };
        num = num / 10;
        len += 1;
    }
    for _ in 0..len {
        let digit: i16;
        unsafe { asm!("pop {}", out(reg) digit) };
        print_char((digit + '0' as i16) as u8);
    }
}

pub fn print_str(s: &str) {
    unsafe {
        asm!(
            "int 21h",
            in("bx") 0x01,
            in("ah") 0x40_u8,
            in("cx") s.len(),
            in("dx") s.as_ptr() as u16,
        )
    }
}

pub fn newline() {
    unsafe { print_char('\n' as u8) };
}

pub fn debug(s: &str, n: i16) {
    unsafe {
        print_str(s);
        print_num(n);
        newline();
    }
}

pub fn println(msg: &str) {
    unsafe {
        print_str(msg);
        newline();
    }
}

pub fn println_bool(f: bool) {
    unsafe {
        if f {
            print_str("true");
        } else {
            print_str("false");
        }
        newline();
    }
}

pub fn read_char() -> u8 {
    unsafe {
        let c: u8 = 0;
        asm!(
            "int 21h",
            in("ah") 0x3f_u8,
            in("bx") 0x00,
            in("cx") 0x01,
            in("dx") &c as *const u8 as u16
        );
        return c;
    }
}

pub fn readline() -> String {
    let mut str = String::empty();
    loop {
        let c = read_char();
        if c == '\n' as u8 || c == '\r' as u8 {
            break;
        }
        str.push(c);
    }
    return str;
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

pub fn timed_readline(timeout_millis: u32) -> String {
    unsafe {
        let mut timeout_byte: u8 = 0;
        set_wait_interval(timeout_millis, &mut timeout_byte as *mut u8);
        let mut str = String::empty();

        while timeout_byte == 0 {
            if let Some(c) = try_get_char() {
                str.push(c as u8);
            }
        }
        return str;
    }
}

pub fn timed_try_get_char(timeout_millis: u32) -> Option<u16> {
    unsafe {
        let mut timeout_byte: u8 = 0;
        let timeout_byte_ptr = &mut timeout_byte as *mut u8;
        set_wait_interval(timeout_millis, timeout_byte_ptr);
        let mut str = String::empty();

        while timeout_byte == 0 {
            if let Some(c) = try_get_char() {
                stop_wait_interval(timeout_byte_ptr);
                return Some(c);
            }
        }
        return None;
    }
}