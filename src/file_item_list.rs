use std::path::Path;

use crate::path_process::pathbuf_to_string_name;

pub mod directory_item;
pub mod file_item;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Kinds {
    File(bool),
    Directory(bool),
}

impl Kinds {
    pub fn classifiy_kinds(path: &Path) -> Self {
        if path.is_dir() {
            Self::Directory(Self::is_hidden(path))
        } else {
            Self::File(Self::is_hidden(path))
        }
    }

    pub fn is_hidden(path: &Path) -> bool {
        let file_item_name = pathbuf_to_string_name(path);
        file_item_name.as_bytes()[0] == b'.'
    }
}
