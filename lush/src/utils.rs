use std::io;
// use std::os::unix::process::CommandExt;
use std::process::Child;
use std::process::Command;

pub fn new_child(cmd: String) -> Result<Child, io::Error> {
    let splited: Vec<&str> = cmd.split(" ").collect();

    return Command::new(splited[0]).arg(splited[1..].join(" ")).spawn();
}
