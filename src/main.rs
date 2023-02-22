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
use panic::exit_with_code;

#[no_mangle]
pub unsafe extern "C" fn start() {
    DOS_ALLOCATOR.zero_memory();
    // debug("DOS_ALLOC[0]: ", DOS_ALLOCATOR.memory[0] as i32);

    let args_len = get_args_len();
    let args = get_args_str(args_len);

    // print_str("Args len: ");
    // print_num(args_len as i32);
    // println();

    // print_str("Arguments: \"");
    // args.print();
    // print_str("\"\n");

    let mut dos_vec1 = DosVec::<u8>::new(5);
    dos_vec1.push('h' as u8);
    dos_vec1.push('e' as u8);
    dos_vec1.push('l' as u8);
    dos_vec1.push('l' as u8);
    let mut dos_vec2 = DosVec::<u8>::new(0);
    dos_vec1.push('o' as u8);
    dos_vec1.push(' ' as u8);
    dos_vec1.push('!' as u8);

    dos_vec2.push('b' as u8);
    dos_vec2.push('y' as u8);
    dos_vec2.push('e' as u8);
    dos_vec2.push('!' as u8);

    debug("vec1 ptr:", dos_vec1.mem_chunk as i32);
    // dos_vec1.clear();
    
    let mut dos_vec3 = DosVec::<u8>::new(3);
    dos_vec3.push('W' as u8);
    dos_vec3.push('T' as u8);
    dos_vec3.push('F' as u8);

    let dv_len1 = dos_vec1.get_len();

    for _ in 0..dv_len1 {
        let c = *dos_vec1.pop().unwrap_unchecked();
        print_char(c);
    }

    print_str("\nDone going through vec1!\n");

    let dv_len3 = dos_vec3.get_len();

    for _ in 0..dv_len3 {
        let c = *dos_vec3.pop().unwrap_unchecked();
        print_char(c);
    }

    print_str("\nDone going through vec3!\n");

    debug("vec1 len: ", dos_vec1.len as i32);
    debug("vec2 len: ", dos_vec2.len as i32);
    debug("vec3 len: ", dos_vec3.len as i32);
    
    debug("vec1 ptr: ", dos_vec1.mem_chunk as i32);
    debug("vec2 ptr: ", dos_vec2.mem_chunk as i32);
    debug("vec3 ptr: ", dos_vec3.mem_chunk as i32);

    exit_with_code(args_len);
}
