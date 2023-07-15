use crate::utils;
use rlua::Lua;

pub fn lua_eval(code: &str) {
    let lua = Lua::new();

    lua.context(|lua_ctx| {
        // let globals = lua_ctx.globals();

        let result = lua_ctx.load(code).exec();

        match result {
            Ok(_) => {}
            Err(_) => {
                let mut child = utils::new_child(code.to_string()).unwrap();

                let status = child.wait().unwrap();
                // println!("{}", status);

                if status.code().unwrap() != 0 {
                    println!("An error occurred.");
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
