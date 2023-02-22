#![no_main]
#![no_std]
#![allow(unused)]

mod dos_vec;
mod io;
mod memory;
mod panic;
mod string;

use core::mem::ManuallyDrop;

use dos_vec::dos_vec::DosVec;
use io::{debug, get_args_len, get_args_str, print_char, print_num, print_str, newline, println};
use memory::dos_allocator::DOS_ALLOCATOR;
use panic::exit_with_code;

use crate::string::string::String;

fn foo(f: impl Fn(i32) -> i32) -> i32 {
    return f(10);
}

#[no_mangle]
pub unsafe extern "C" fn start() {
    DOS_ALLOCATOR.zero_memory();


    let args_len = get_args_len();
    let args = get_args_str(args_len);

    print_str("Args len: ");
    print_num(args_len as i32);
    newline();

    print_str("Arguments: \"");
    args.print();
    print_str("\"\n");

    let words = String::from_str("a, a");
    let mut words_split = words.split(|c: u8| c == ',' as u8 || c == ' ' as u8);

    // print_str("Arguments split: \n");

    debug("Words split len: ", words_split.get_len() as i32);

    for i in 0..words_split.get_len() {
        words_split[i].truncate(|c: u8| c == ' ' as u8 || c == ',' as u8);
        words_split[i].print();
        newline();
    }

    exit_with_code(args_len);
}
