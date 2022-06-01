use std::{fs::Metadata, path::PathBuf};

use super::Kinds;

#[derive(Debug, Clone)]
pub struct FileItem {
    name: String,
    path: PathBuf,
    meta: Metadata,
    kinds: Kinds,
}

impl FileItem {
    pub fn new(name: String, path: PathBuf, meta: Metadata, kinds: Kinds) -> Self {
        Self {
            name,
            path,
            meta,
            kinds,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn kinds(&self) -> Kinds {
        self.kinds.clone()
    }
}
