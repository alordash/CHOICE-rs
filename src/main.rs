#![no_main]
#![no_std]
#![allow(unused)]

mod dos;
mod dos_vec;
mod io;
mod memory;
mod panic;
mod string;

use core::mem::ManuallyDrop;

use dos::{get_stdin_status, set_wait_interval, direct_console_input, wait};
use dos_vec::dos_vec::DosVec;
use io::{
    debug, get_args_len, get_args_str, newline, print_char, print_num, print_str, println,
    read_char, read_string,
};
use memory::dos_allocator::DOS_ALLOCATOR;
use panic::exit_with_code;

use crate::{io::println_bool, string::string::String};

fn foo(f: impl Fn(i32) -> i32) -> i32 {
    return f(10);
}

#[no_mangle]
pub unsafe extern "C" fn start() {
    DOS_ALLOCATOR.zero_memory();

    // loop {
    //     if get_stdin_status() > 0 {
    //         direct_console_input();
    //     }
    // }

    let args_len = get_args_len();
    let args = get_args_str(args_len);

    print_str("Args len: ");
    print_num(args_len as i32);
    newline();

    print_str("Arguments: \"");
    args.print();
    print_str("\"\n");

    let mut words_split = args.split(|c: u8| c == ',' as u8 || c == ' ' as u8);
    for i in 0..words_split.get_len() {
        words_split[i].truncate(|c: u8| c == ' ' as u8 || c == ',' as u8);
    }
    words_split.remain_filtered(|str: &String| !str.is_empty());

    print_str("Arguments split: \n");
    for i in 0..words_split.get_len() {
        words_split[i].print();
        newline();
    }

    let mut byte: u8 = 0;

    debug("BEGIN byte: ", byte as i32);
    set_wait_interval(1000, &mut byte as *mut u8);
    // wait(1000);
    debug("MID byte: ", byte as i32);

    let read_str = read_string();

    let maybe_idx = words_split.find_idx(move |str| str == &read_str);
    if let Some(idx) = maybe_idx {
        print_str("Entered option #");
        print_num(idx as i32);
        newline();
    } else {
        println("Wrong option");
    }

    debug("END byte: ", byte as i32);

    exit_with_code(args_len);
}
