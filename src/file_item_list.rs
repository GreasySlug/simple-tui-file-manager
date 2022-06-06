use std::path::PathBuf;

use crate::path_process::pathbuf_to_string_name;

pub mod directory_item;
pub mod file_item;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Kinds {
    File = 1,
    Directory = 2,
}

impl Kinds {
    fn classifiy_kinds(path: &std::path::Path) -> Self {
        if path.is_dir() {
            Self::Directory
        } else {
            Self::File
        }
    }

    fn is_hidden(path: &PathBuf) -> bool {
        let file_item_name = pathbuf_to_string_name(path);
        file_item_name.as_bytes()[0] == b'.'
    }
}
