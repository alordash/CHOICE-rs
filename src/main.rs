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
    // debug("DOS_ALLOC[0]: ", DOS_ALLOCATOR.memory[0] as i32);

    let words = String::from_str("one, two, word, another, end");
    let words_split = words.split(|c: u8| c == ',' as u8 || c == 'o' as u8);

    print_str("Words: ");
    words.print();
    newline();

    for i in 0..words_split.get_len() {
        words_split[i].print();
        newline();
    }

    fn asd(v: i32) -> i32 {
        v + 5
    }

    // let q = |v: i32| v + 66;

    let v = foo(|v: i32| v + 32);

    print_num(v);

    let args_len = get_args_len();
    // let args = get_args_str(args_len);

    // print_str("Args len: ");
    // print_num(args_len as i32);
    // println();

    // print_str("Arguments: \"");
    // args.print();
    // print_str("\"\n");

    exit_with_code(args_len);
}
