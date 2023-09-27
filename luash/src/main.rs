mod buildin;
mod cmd;
mod lua;

use std::env;
// use std::io::{stdin, stdout, Write};
// use std::fmt;
use std::path::Path;

use promkit::{build::Builder, crossterm::style, readline};

use whoami;

fn main() {
    // lua::lua::lua_eval(r#"print("Hi!")"#);

    let binding = env::current_dir().unwrap();
    let pwd = binding.parent().unwrap();

    sh(pwd);
}

fn sh(pwd: &Path) {
    let title = format!(
        "{}@{}:{}",
        whoami::username(),
        whoami::hostname(),
        pwd.display(),
    );

    let mut p = readline::Builder::default()
        .title(title)
        .title_color(style::Color::DarkGreen)
        .build()
        .unwrap();

    loop {
        // print!(
        //     "{}@{}:{} > ",
        //     whoami::username(),
        //     whoami::hostname(),
        //     pwd.display()
        // );

        let line = p.run();

        // stdout().flush().unwrap();

        // let mut line = String::new();
        // stdin().read_line(&mut line).expect("Faild to read line");
        // line.remove(line.len() - 1);

        match line {
            Ok(line) => lua::lua::lua_eval(&line),
            Err(err) => println!("{}", err),
        }
    }
}
