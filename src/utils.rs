use crate::string::string::String;

pub fn try_extract_i32_from_str_offset(s: &String, offset: usize) -> Option<i32> {
    let num_str = s.substring(offset, s.get_len());
    num_str.try_to_i32()
}
