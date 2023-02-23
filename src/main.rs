#![no_main]
#![no_std]
#![allow(unused)]

mod dos;
mod dos_vec;
mod io;
mod memory;
mod panic;
mod string;
mod utils;

use core::mem::ManuallyDrop;

use dos::{set_wait_interval, wait};
use dos_vec::dos_vec::DosVec;
use io::{
    debug, get_args_len, get_args_str, newline, print_char, print_num, print_str, println,
    read_char, readline, timed_readline, timed_try_get_char, try_get_char,
};
use memory::dos_allocator::DOS_ALLOCATOR;
use panic::exit_with_code;
use utils::try_extract_i32_from_str_offset;

use crate::{io::println_bool, string::string::String};

const TIMEOUT_LITERAL: &'static str = "timeout=";
const DEFAULT_LITERAL: &'static str = "default=";

const DEFAULT_RESULT: u8 = 0;

#[no_mangle]
pub unsafe extern "C" fn start() {
    DOS_ALLOCATOR.zero_memory();

    let args_len = get_args_len();
    let args_str = get_args_str(args_len);

    let mut args = args_str.split(|c: u8| c == ',' as u8 || c == ' ' as u8);
    for i in 0..args.get_len() {
        args[i].truncate(|c: u8| c == ' ' as u8 || c == ',' as u8);
    }
    args.remain_filtered(|str: &String| !str.is_empty());

    let timeout_seconds = {
        if let Some(idx) = args.find_idx(|arg| arg.begins_with(&String::from_str(TIMEOUT_LITERAL)))
        {
            let timeout_str = &args[idx];
            try_extract_i32_from_str_offset(&timeout_str, TIMEOUT_LITERAL.len())
                .map(|v| 1000 * v as u32)
        } else {
            None
        }
    };

    let default = if let Some(idx) =
        args.find_idx(|arg| arg.begins_with(&String::from_str(DEFAULT_LITERAL)))
    {
        let default_str = &args[idx];
        default_str.substring(DEFAULT_LITERAL.len(), default_str.get_len())
    } else {
        String::empty()
    };

    let input = if let Some(t) = timeout_seconds {
        timed_readline(t)
    } else {
        readline()
    };

    let search_term = if input.get_len() == 0 { default } else { input.clone() };
    let result = args
        .find_idx(move |arg| arg == &search_term)
        .map(|v| v as u8)
        .or(Some(DEFAULT_RESULT))
        .unwrap_unchecked() as u8;
    // print_str("Entered: \"");
    // input.print();
    // print_str("\"\n");
    // debug("Result: ", result as i16);

    exit_with_code(result);
}
