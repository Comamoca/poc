use crate::buildin::buildin;
use crate::utils;
use rlua::Lua;

pub fn lua_eval(code: &str) {
    let lua = Lua::new();

    // HACK: move program to execute shell code to outside.
    lua.context(|lua_ctx| {
        // let globals = lua_ctx.globals();

        let result = lua_ctx.load(code).exec();

        match result {
            // if input string is lua code.
            Ok(_) => {}

            // if isnt
            Err(_) => {
                match code {
                    "ls" => {
                        // HACK: Combine duplicate operations
                        buildin::ls(code);
                    }
                    _ => {
                        let mut child = utils::new_child(code.to_string()).unwrap();

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

                // println!("{}", result);

                // match result {
                //     Ok(output) => {
                //         println!("{}", String::from_utf8_lossy(&output.stdout))
                //     }
                //     Err(err) => println!("{}", err),
                // }
            }
        }
    });
}
