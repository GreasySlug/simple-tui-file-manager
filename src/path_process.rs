use std::path::PathBuf;
use std::{env::current_dir, path::Path};

use crate::file_item_list::file_item::Extension;
use crate::file_item_list::{file_item::FileItem, Kinds};

pub fn pathbuf_to_string_name(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn make_info_files_from_dirpath(path: &Path) -> Vec<FileItem> {
    let mut file_items_vec: Vec<FileItem> = Vec::new();

    if let Ok(dir) = path.read_dir() {
        for entry in dir {
            let entry = entry.unwrap();
            let file_path = entry.path();
            let file_item = make_a_info_files_from_dirpath(&file_path);

            if let Some(item) = file_item {
                file_items_vec.push(item);
            }
        }
    }
    file_items_vec
}

pub fn make_a_info_files_from_dirpath(file_path: &Path) -> Option<FileItem> {
    let file_name = pathbuf_to_string_name(file_path);
    let meta = file_path.metadata();
    let kinds = Kinds::classifiy_kinds(file_path);
    let hidden = Kinds::is_hidden(file_path);
    let extension = if kinds == Kinds::Directory(true) || hidden {
        None
    } else {
        Some(Extension::classify_extension(file_path))
    };
    if let Ok(meta) = meta {
        return Some(FileItem::new(
            file_name,
            file_path.to_path_buf(),
            meta,
            kinds,
            extension,
        ));
    }
    None
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
