use crate::{string::string::String, io::{println, print_str}};

pub fn try_extract_i32_from_str_offset(s: &String, offset: usize) -> Option<i32> {
    unsafe {
        let num_str = s.substring(offset, s.get_len());
        num_str.try_to_i32()
    }
}
