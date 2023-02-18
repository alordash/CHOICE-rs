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
        "mov ah, 2",
        "int 21h",
        in("dl") c as u8
    );
}
