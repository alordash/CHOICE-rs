#![no_main]
#![no_std]

mod io;
mod panic;
mod string;

use core::arch::asm;

use io::{get_args_count, print_char, get_args_str};

unsafe fn exit_with_code(exit_code: u8) {
    asm!(
        "mov ah, 4Ch",
        "int 21h",
        in("al") exit_code,
    )
}

#[no_mangle]
pub unsafe extern "C" fn start() {
    let args_count = get_args_count();
    let args = get_args_str(args_count);

    print_char((args_count + 48) as char);
    print_char('\n');
    print_char('\n');
    print_char('\n');
    // print_char((args_count + 48) as char);
    print_char('\n');
    args.print_v2();
    // args.print();

    // print_char((argv_ptr + 48) as u8 as char);

    exit_with_code(args_count);
}
