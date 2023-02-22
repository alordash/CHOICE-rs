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
use io::{debug, get_args_len, get_args_str, newline, print_char, print_num, print_str, println, read_string, read_char};
use memory::dos_allocator::DOS_ALLOCATOR;
use panic::exit_with_code;

use crate::{io::println_bool, string::string::String};

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

    // let mut words_split = args.split(|c: u8| c == ',' as u8 || c == ' ' as u8);
    // // debug("Words split len before: ", words_split.get_len() as i32);

    // for i in 0..words_split.get_len() {
    //     words_split[i].truncate(|c: u8| c == ' ' as u8 || c == ',' as u8);
    // }

    // words_split.remain_filtered(|str: &String| !str.is_empty());
    // // debug("Words split len after:  ", words_split.get_len() as i32);

    // print_str("Arguments split: \n");

    // for i in 0..words_split.get_len() {
    //     words_split[i].print();
    //     newline();
    // }

    let read_str = read_string();
    print_str("Read str:\n");
    read_str.print();

    exit_with_code(args_len);
}
