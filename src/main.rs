#![no_main]
#![no_std]

mod dos_vec;
mod io;
mod memory;
mod panic;
mod string;

use core::mem::size_of;

use dos_vec::dos_vec::DosVec;
use io::{debug, get_args_len, get_args_str, print_char, print_num, print_str, println};
use memory::{dos_allocator::DOS_ALLOCATOR, memory_chunk::MemChunk};
use panic::exit_with_code;

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

    exit_with_code(args_len);
}
