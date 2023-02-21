#![no_main]
#![no_std]

mod dos_vec;
mod io;
mod memory;
mod panic;
mod string;

use core::{arch::asm, cell::UnsafeCell};

use dos_vec::dos_vec::DosVec;
use io::{debug, get_args_len, get_args_str, print_char, print_num, print_str, println};
use memory::dos_allocator::DOS_ALLOCATOR;

unsafe fn exit_with_code(exit_code: u8) {
    asm!(
        "mov ah, 4Ch",
        "int 21h",
        in("al") exit_code,
    )
}

#[no_mangle]
pub unsafe extern "C" fn start() {
    DOS_ALLOCATOR.zero_memory();
    // debug("DOS_ALLOC[0]: ", DOS_ALLOCATOR.memory[0] as i32);

    let args_len = get_args_len();
    let args = get_args_str(args_len);

    print_str("Args len: ");
    print_num(args_len as i32);
    println();

    print_str("Arguments: \"");
    args.print();
    print_str("\"\n");

    let mut dos_vec = DosVec::<u8>::new(5);
    dos_vec.push('h' as u8);
    dos_vec.push('e' as u8);
    dos_vec.push('l' as u8);
    dos_vec.push('l' as u8);
    dos_vec.push('o' as u8);
    dos_vec.push(' ' as u8);
    dos_vec.push('!' as u8);

    print_str("vec len: ");
    print_num(dos_vec.get_len() as i32);
    println();

    let dv_len = dos_vec.get_len();

    for _ in 0..dv_len {
        let c = *dos_vec.pop().unwrap_unchecked();
        print_char(c);
    }

    print_str("\nDone going through vec!\n");

    exit_with_code(args_len);
}
