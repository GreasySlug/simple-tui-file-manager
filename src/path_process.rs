use std::env::current_dir;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use crate::application::App;
use crate::file_item_list::file_item::FileItem;
use crate::file_item_list::Kinds;

pub fn pathbuf_to_string_name(path: &Path) -> String {
    let path_name = path.file_name();
    if let Some(name) = path_name {
        return name.to_str().unwrap().to_string();
    }
    "root".to_string()
}

pub fn make_file_items_from_dirpath(path: &Path) -> Vec<FileItem> {
    let mut files_item: Vec<FileItem> = Vec::new();

    if let Ok(dir) = path.read_dir() {
        for entry in dir {
            let entry = entry.unwrap();
            let file_path = entry.path();
            let file_name = pathbuf_to_string_name(&file_path);
            let meta = entry.metadata();
            if meta.is_err() {
                continue;
            }
            let meta = meta.unwrap();
            let kinds = Kinds::classify_kinds(path, &meta);
            // let hidden = Kinds::is_hidden(path);
            files_item.push(FileItem::new(file_name, file_path, meta, kinds));
        }
    }

    files_item
}

pub fn make_a_file_item_from_dirpath(file_path: &Path) -> Option<FileItem> {
    let file_name = pathbuf_to_string_name(file_path);
    let meta = file_path.metadata();
    if meta.is_err() {
        return None;
    }
    let meta = meta.unwrap();
    let kinds = Kinds::classify_kinds(file_path, &meta);
    // let hidden = Kinds::is_hidden(file_path);
    Some(FileItem::new(
        file_name,
        file_path.to_path_buf(),
        meta,
        kinds,
    ))
}

pub fn working_dir_path() -> PathBuf {
    match current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Permission denied: {}", e),
    }
}

// C:\Users\UserName\Downloads\
#[cfg(target_os = "windows")]
pub fn get_user_profile_path(additional_path: &str) -> Option<PathBuf> {
    match std::env::var("USERPROFILE") {
        Ok(mut path) => {
            path.push('\\');
            path.push_str(additional_path);
            Some(PathBuf::from(path))
        }
        Err(e) => None,
    }
}

//  /home/userName
#[cfg(target_os = "linux")]
pub fn get_user_profile_path(additional_path: &str) -> Option<PathBuf> {
    match std::env::var("HOME") {
        Ok(mut path) => {
            path.push('/');
            path.push_str(additional_path);
            Some(PathBuf::from(path))
        }
        Err(e) => None,
    }
}

pub fn join_to_crr_dir(app: &mut App, relpath: impl AsRef<Path>) -> PathBuf {
    let fullpath = app
        .selecting_statefuldir_mut()
        .directory()
        .pathbuf()
        .join(relpath);
    fullpath
}

// arguments can only take paths
pub fn user_commands(cmd: &str, args: Vec<&Path>) -> io::Result<()> {
    if let Ok(mut child) = std::process::Command::new(cmd).args(args).spawn() {
        child.wait()?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::path_process::pathbuf_to_string_name;

    #[test]
    fn pathbuf_to_string_name_test() {
        let items = [
            (".git", ".git"),
            (".config/nvim", "nvim"),
            (".config/rofi", "rofi"),
            ("init.vim", "init.vim"),
        ];

        for (path, name) in items.into_iter() {
            let path_name = pathbuf_to_string_name(Path::new(path));
            assert_eq!(path_name, name);
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn home_directory_path_test() {
        use std::env;
        let key = "HOME";
        match env::var(key) {
            Ok(val) => println!("{key}: {val:?}"),
            Err(e) => println!("couldn't interpret {key}: {e}"),
        }
    }
}
