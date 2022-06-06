use std::path::PathBuf;
use std::{env::current_dir, path::Path};

use crate::file_item_list::{file_item::FileItem, Kinds};

pub fn pathbuf_to_string_name(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn make_info_files_from_dirpath(path: &Path) -> Vec<FileItem> {
    let mut files_item: Vec<FileItem> = Vec::new();

    if let Ok(dir) = path.read_dir() {
        for entry in dir {
            let entry = entry.unwrap();
            let file_path = entry.path();
            let file_name = pathbuf_to_string_name(&file_path);
            let meta = entry.metadata().unwrap();
            let kinds = Kinds::classifiy_kinds(path);
            let hidden = Kinds::is_hidden(&path.to_path_buf());
            files_item.push(FileItem::new(file_name, file_path, meta, kinds, hidden));
        }
    }

    files_item
}

pub fn make_a_info_files_from_dirpath(path: &Path) -> FileItem {
    let name = pathbuf_to_string_name(path);
    let meta = path.metadata().expect("Failed to get metadata");
    let kinds = Kinds::classifiy_kinds(path);
    let hidden = Kinds::is_hidden(&path.to_path_buf());
    FileItem::new(name, path.to_path_buf(), meta, kinds, hidden)
}

pub fn get_current_dir_path() -> PathBuf {
    match current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Permission denide: {}", e),
    }
}

//  C:\Users\UserNmae
#[cfg(target_os = "windows")]
pub fn get_home_directory_path() -> Option<PathBuf> {
    let home_dir_name = "USERPROFILE";
    match std::env::var(home_dir_name) {
        Ok(path) => Some(PathBuf::from(path)),
        Err(e) => None,
    }
}

//  /home/userName
#[cfg(target_os = "linux")]
pub fn get_home_directory_path() -> Option<PathBuf> {
    let home_dir = "HOME";
    match std::env::var(home_dir) {
        Ok(path) => Some(PathBuf::from(path)),
        Err(e) => None,
    }
}
