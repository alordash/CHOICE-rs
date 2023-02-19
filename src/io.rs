use core::arch::asm;

use crate::string::string_wrapper::StringWrapper;

pub unsafe fn get_args_count() -> u8 {
    let args_count: u8;
    asm!(
        "xor cx, cx",
        "mov cl, byte ptr ds:[80h]",
        out("cl") args_count
    );
    args_count
}

pub unsafe fn get_args_str(args_count: u8) -> StringWrapper {
    let argv_ptr: u8;
    asm!(
        "xor cx, cx",
        "lea cx, byte ptr ds:[81h]",
        out("cl") argv_ptr
    );
    StringWrapper::from_raw_parts(argv_ptr as *const u8, args_count as usize)
}

pub unsafe fn print_char(c: char) {
    asm!(
        "int 21h",
        in("ah") 2_u8,
        in("dl") c as u8
    );
}

pub unsafe fn print_number(mut num: i32) {
    if num.is_negative() {
        print_char('-');
        num = -num;
    }
    let mut len = 0;
    while num > 0 {
        let rem = num % 10;
        asm!("push {}", in(reg) rem);
        num = num / 10;
        len += 1;
    }
    for _ in 0..len {
        let digit: i32;
        asm!("pop {}", out(reg) digit);
        print_char((digit + '0' as i32) as u8 as char);
    }
}
