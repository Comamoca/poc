use crate::buildin::buildin;

pub fn ls(args: &str) {
    let splited: Vec<&str> = args.split(" ").collect();

    if splited.len() < 2 {
        // execute `ls`
        let files = buildin::get_file_list(".");

        println!("{}", files.join("\n"));
    }

    let files = buildin::get_file_list(splited[0]);

    println!("{}", files.join("\n"));
}
