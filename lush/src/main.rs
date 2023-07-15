mod lua;
mod utils;

use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;

use whoami;

fn main() {
    // lua::lua::lua_eval(r#"print("Hi!")"#);

    let binding = env::current_dir().unwrap();
    let pwd = binding.parent().unwrap();

    sh(pwd);
}

fn sh(pwd: &Path) {
    loop {
        print!(
            "{}@{}:{} > ",
            whoami::username(),
            whoami::hostname(),
            pwd.display()
        );

        stdout().flush().unwrap();

        let mut line = String::new();
        stdin().read_line(&mut line).expect("Faild to read line");
        line.remove(line.len() - 1);

        lua::lua::lua_eval(&line);
    }
}
