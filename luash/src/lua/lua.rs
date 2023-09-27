use crate::cmd;
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
                cmd::exec_shell(code)

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
