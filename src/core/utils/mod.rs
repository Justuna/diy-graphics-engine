use std::ffi::CString;

pub fn empty_cstring(len: usize) -> CString
{
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));

    return unsafe { CString::from_vec_unchecked(buffer) };
}