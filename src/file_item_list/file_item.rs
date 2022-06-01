use std::{
    fs::Metadata,
    path::{Path, PathBuf},
};

use crate::path_process::pathbuf_to_string_name;

use super::Kinds;

#[derive(Debug, Clone)]
pub struct FileItem {
    name: String,
    path: PathBuf,
    meta: Metadata,
    kinds: Kinds,
}

impl FileItem {
    pub fn new(path: PathBuf, meta: Metadata, kinds: Kinds) -> Self {
        let name = pathbuf_to_string_name(&path);
        let kinds = determin_kinds_of_file_item(path.as_path());
        Self {
            name,
            path,
            meta,
            kinds,
        }
    }
}
fn determin_kinds_of_file_item(path: &Path) -> Kinds {
    if path.is_dir() {
        Kinds::Directory
    } else {
        Kinds::File
    }
}
