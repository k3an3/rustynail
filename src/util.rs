use std::os::raw::c_void;
use std::ffi::{CStr, CString};

pub fn from_c_str(buf: *mut c_void) -> String {
    unsafe {CStr::from_ptr(buf as *const _)}.to_string_lossy().into_owned()
}

pub fn c_str(buf: &str) -> CString {
    CString::new(buf).unwrap()
}

pub fn split_string<'a>(s: &'a str, split: &str, offset: usize) -> &'a str {
    s.split(split).collect::<Vec<&str>>()[offset]
}
