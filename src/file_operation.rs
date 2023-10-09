use std::{
    fs::{self},
    path::{Path, PathBuf},
};

pub fn read_folder(path: &Path) -> Option<Vec<PathBuf>> {
    let file_names = fs::read_dir(path).expect("error: can't open folder");

    let mut arr_path = Vec::new();
    for file_name in file_names {
        let file_name = file_name.expect("can't open file");
        let path_file = file_name.path();

        if path_file.is_file() {
            let extension = path_file.extension().and_then(std::ffi::OsStr::to_str);

            match extension {
                Some("jpg") | Some("jpeg") | Some("png") | Some("webp") => arr_path.push(path_file),
                _ => (),
            }
        }
    }

    if !arr_path.is_empty() {
        Some(arr_path)
    } else {
        None
    }
}
