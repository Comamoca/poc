use std::fs;
// use std::io;
// use std::string::String;

pub fn get_file_list(path: &str) -> Vec<String> {
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
