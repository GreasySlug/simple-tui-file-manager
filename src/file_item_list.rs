use std::{fs::Metadata, path::PathBuf};

#[derive(Debug, Clone)]
enum Kinds {
    File = 1,
    Directory = 2,
    Hidden = 3,
    Dot = 4,
}

#[derive(Debug, Clone)]
struct FileItem {
    name: String,
    path: PathBuf,
    meta: Metadata,
    kinds: Kinds,
}

impl FileItem {
    pub fn new(path: PathBuf, meta: Metadata) -> Self {
        let kinds = determin_kinds_of_file_item(path.as_path());
        let name = pathbuf_to_string_name(&path);
        Self {
            name,
            path,
            meta,
            kinds,
        }
    }
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    path: PathBuf,
}

impl Directory {
    pub fn new(path: PathBuf) -> Self {
        let name = pathbuf_to_string_name(&path);
        Self { name, path }
    }
}

fn pathbuf_to_string_name(path: &PathBuf) -> String {
    path.as_path().into_string()
}

fn determin_kinds_of_file_item(path: &Path) -> Kinds {
    if path.is_dir() {
        Kinds::Directory
    } else {
        Kinds::File
    }
}
