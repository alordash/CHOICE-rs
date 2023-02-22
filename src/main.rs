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

fn display_dos_vec(dv: &DosVec<String>) {
    unsafe {
        for i in 0..dv.get_len() {
            // print_num(*dv.buf_ptr.add(i) as i32);
            // newline();
            dv[i].print();
            newline();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn start() {
    DOS_ALLOCATOR.zero_memory();
    // debug("DOS_ALLOC[0]: ", DOS_ALLOCATOR.memory[0] as i32);

    let words = String::from_str("one, two, word, another, end");
    let words_split = words.split(',' as u8);

    print_str("Words: ");
    words.print();
    newline();

    for i in 0..words_split.get_len() {
        words_split[i].print();
        newline();
    }

    // let mut vec = DosVec::<u8>::new(6);
    // vec.push(&('w' as u8));
    // vec.push(&('T' as u8));
    // vec.grow(10);
    // vec.push(&('f' as u8));

    // for i in 0..vec.get_len() {
    //     print_char(vec[i]);
    // }
    // newline();

    // let mut small_vec = DosVec::<u8>::new(6);
    // small_vec.push(&('l' as u8));

    // for i in 0..small_vec.get_len() {
    //     print_char(small_vec[i]);
    // }
    // newline();

    // debug("VEC PTR:       ", vec.mem_chunk as i32);
    // debug("SMALL VEC PTR: ", small_vec.mem_chunk as i32);

    // let mut strs = DosVec::<String>::new(1);
    // debug("Self ptr: ", strs.mem_chunk as i32);
    // newline();

    // // let str1 = String::from_str("FIRST LONGEST STR 1");

    // strs.push(String::from_str("FIRST LONGEST STR 1"));
    // debug("Self ptr: ", strs.mem_chunk as i32);
    // debug("First ptr: ", strs[0].mem_chunk as i32);
    // display_dos_vec(&strs);
    // newline();

    // strs.push(String::from_str("SECOND STR"));
    // debug("Self ptr: ", strs.mem_chunk as i32);
    // debug("First ptr: ", strs[0].mem_chunk as i32);
    // debug("Second ptr: ", strs[1].mem_chunk as i32);
    // display_dos_vec(&strs);

    // strs.push(String::from_str("THE 3rd STR"));
    // debug("Self ptr: ", strs.mem_chunk as i32);
    // debug("First ptr: ", strs[0].mem_chunk as i32);
    // debug("Second ptr: ", strs[1].mem_chunk as i32);
    // debug("Third ptr: ", strs[2].mem_chunk as i32);
    // display_dos_vec(&strs);
    // newline();

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
