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

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    path: PathBuf,
}
