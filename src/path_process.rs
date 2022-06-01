use std::env::current_dir;
use std::path::PathBuf;

use crate::file_item_list::{FileItem, Kinds};

pub fn pathbuf_to_string_name(path: &PathBuf) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn make_dirpath_info_files_vec(path: PathBuf) -> Vec<FileItem> {
    let files_item: Vec<FileItem> = Vec::new();

    if let Ok(dir) = path.read_dir() {
        for entry in dir {
            let entry = entry.unwrap();
            let file_path = entry.path();
            let file_name = pathbuf_to_string_name(&file_path);
            let meta = entry.metadata().unwrap();
            let kinds = if file_path.is_dir() {
                Kinds::Directory
            } else {
                Kinds::File
            };
            files_item.push(FileItem::new(file_name, file_path, meta, kinds));
        }
    }

    files_item
}

pub fn get_current_dir_path() -> PathBuf {
    match current_dir() {
        Ok(path) => return path,
        Err(e) => panic!("Permission denide: {}", e),
    }
}
