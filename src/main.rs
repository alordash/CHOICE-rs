#![no_main]
#![no_std]
#![allow(unused)]

mod dos_vec;
mod io;
mod memory;
mod panic;
mod string;

use dos_vec::dos_vec::DosVec;
use io::{debug, get_args_len, get_args_str, print_char, print_num, print_str, println};
use memory::dos_allocator::DOS_ALLOCATOR;
use panic::exit_with_code;

use crate::string::string::String;

#[no_mangle]
pub unsafe extern "C" fn start() {
    DOS_ALLOCATOR.zero_memory();
    // debug("DOS_ALLOC[0]: ", DOS_ALLOCATOR.memory[0] as i32);

    const strs_len: usize = 10;
    let mut strs = DosVec::<DosVec<u8>>::new(strs_len);
    for i in 0..strs_len {
        strs.push(DosVec::new(strs_len));
        strs[i].push('s' as u8);
        strs[i].push('t' as u8);
        strs[i].push('r' as u8);
        strs[i].push(' ' as u8);
        strs[i].push('#' as u8);
        strs[i].push(i as u8 + '0' as u8);

        print_str("Str #");
        print_num(i as i32);
        print_str(" len: ");
        print_num(strs[i].get_len() as i32);
        println();
    }

    debug("Strs len: ", strs.get_len() as i32);

    for i in 0..strs.get_len() {
        let str = String::from_vec(&strs[i]);
        str.print();
        println();
    }

    let args_len = get_args_len();
    let args = get_args_str(args_len);

    print_str("Args len: ");
    print_num(args_len as i32);
    println();

    print_str("Arguments: \"");
    args.print();
    print_str("\"\n");

    exit_with_code(args_len);
}
