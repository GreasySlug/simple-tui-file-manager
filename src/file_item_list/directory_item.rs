use std::path::PathBuf;

use crate::path_process::pathbuf_to_string_name;

#[derive(Debug, Clone)]
pub struct Directory {
    name: String,
    path: PathBuf,
}

impl Directory {
    pub fn new(path: PathBuf) -> Self {
        let name = pathbuf_to_string_name(&path);
        Self { name, path }
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}
