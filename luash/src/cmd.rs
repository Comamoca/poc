//! コマンドを実行する処理が書かれています。
//! `exec_shell`で任意のコマンドを
//! なお、Lua以外のコマンドが入力されたときに実行されるフォールバック処理

use std::io;
// use std::os::unix::process::CommandExt;
use crate::buildin;

use std::process::Child;
use std::process::Command;

fn new_child(cmd: Vec<String>) -> Result<Child, io::Error> {
    return Command::new(cmd[0].to_owned())
        .arg(cmd[1..].join(" "))
        .spawn();
}

fn parse(input: &str) -> Vec<String> {
    let result: Vec<&str> = input.trim().split(" ").collect();
    return result.iter().map(|&s| s.to_string()).collect();
}

pub fn exec_shell(code: &str) {
    match code {
        "ls" => {
            // HACK: Combine duplicate operations
            buildin::ls::ls(code)
        }
        "exit" => buildin::exit::exit(),
        _ => {
            let mut child = new_child(parse(code)).unwrap();

            let status = child.wait();

            match status {
                Ok(status) => {
                    if status.code().unwrap() != 0 {
                        println!("An error occurred at execute shell command.");
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
    }
}
