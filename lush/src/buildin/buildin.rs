use std::fs;
// use std::io;
// use std::string::String;

fn get_file_list(path: &str) -> Vec<String> {
    // ディレクトリ内のファイル一覧を取得
    if let Ok(entries) = fs::read_dir(path) {
        let mut files = Vec::new();

        for entry in entries {
            if let Ok(entry) = entry {
                let fname = entry.file_name();

                if let Some(file_name) = &fname.to_str() {
                    files.push(file_name.to_string());
                }
            }
        }
        return files;
    }

    Vec::new()
}

pub fn ls(args: &str) {
    let splited: Vec<&str> = args.split(" ").collect();

    if splited.len() < 2 {
        // execute `ls`
        let files = get_file_list(".");

        println!("{}", files.join("\n"));
    }

    let files = get_file_list(splited[0]);

    println!("{}", files.join("\n"));
}
