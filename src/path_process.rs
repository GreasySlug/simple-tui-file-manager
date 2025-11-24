use std::env::current_dir;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use crate::application::App;
use crate::file_item_list::file_item::Extension;
use crate::file_item_list::file_item::FileItem;
use crate::file_item_list::Kinds;

pub fn pathbuf_to_string_name(path: &Path) -> String {
    let path_name = path.file_name();
    if let Some(name) = path_name {
        return name.to_str().unwrap().to_string();
    }
    "root".to_string()
}

pub fn make_info_files_from_dirpath(path: &Path) -> Vec<FileItem> {
    let mut files_item: Vec<FileItem> = Vec::new();

    if let Ok(dir) = path.read_dir() {
        for entry in dir {
            let entry = entry.unwrap();
            let file_path = entry.path();
            let file_name = pathbuf_to_string_name(&file_path);
            let meta = entry.metadata().unwrap();
            let kinds = Kinds::classifiy_kinds(path, &meta);
            let hidden = Kinds::is_hidden(path);
            let extension = if kinds == Kinds::Directory(true) || hidden {
                None
            } else {
                Some(Extension::classify_extension(&file_path))
            };
            files_item.push(FileItem::new(file_name, file_path, meta, kinds, extension));
        }
    }

    files_item
}

pub fn make_a_info_files_from_dirpath(file_path: &Path) -> FileItem {
    let file_name = pathbuf_to_string_name(file_path);
    let meta = file_path.metadata().expect("Failed to get metadata");
    let kinds = Kinds::classifiy_kinds(file_path, &meta);
    let hidden = Kinds::is_hidden(file_path);
    let extension = if kinds == Kinds::Directory(true) || hidden {
        None
    } else {
        Some(Extension::classify_extension(file_path))
    };
    FileItem::new(file_name, file_path.to_path_buf(), meta, kinds, extension)
}

pub fn current_dir_path() -> PathBuf {
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

//  /home/userName or /Users/userName (macOS)
#[cfg(not(target_os = "windows"))]
pub fn get_home_directory_path() -> Option<PathBuf> {
    let home_dir = "HOME";
    match std::env::var(home_dir) {
        Ok(path) => Some(PathBuf::from(path)),
        Err(e) => None,
    }
}

pub fn create_dir_by_relpath(app: &mut App, relpath: impl AsRef<Path>) -> io::Result<()> {
    let fullpath = app
        .peek_selected_statefuldir()
        .directory()
        .pathbuf()
        .join(relpath);
    std::fs::create_dir_all(fullpath)
}
