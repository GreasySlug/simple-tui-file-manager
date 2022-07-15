use std::{fs::Metadata, path::Path};

use crate::path_process::pathbuf_to_string_name;

pub mod directory_item;
pub mod file_item;

// true is hidden file item
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Kinds {
    File(bool),
    Directory(bool),
}

impl Kinds {
    pub fn classifiy_kinds(path: &Path, meta: &Metadata) -> Self {
        if path.is_file() || meta.is_file() {
            Self::File(Self::is_hidden(path))
        } else {
            Self::Directory(Self::is_hidden(path))
        }
    }

    pub fn is_hidden(path: &Path) -> bool {
        let file_item_name = pathbuf_to_string_name(path);
        let first = file_item_name.chars().position(|c| c == '.');
        if first.is_none() {
            false
        } else {
            first == Some(0)
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::file_item_list::Kinds;

    #[test]
    fn is_hidden_file_item() {
        let items = [".git", ".vscode", ".vimrc.old", ".profile"];

        for name in items.into_iter() {
            let path = Path::new(name);
            assert!(Kinds::is_hidden(path));
        }
    }

    #[test]
    fn is_not_hidden_file_item() {
        let items = ["git", "init.vim", "vimrc.old", "sample.txt"];

        for name in items.into_iter() {
            let path = Path::new(name);
            assert!(!Kinds::is_hidden(path));
        }
    }

    #[test]
    fn crr_dir_items() {
        let path = std::env::current_dir();
        if let Ok(ref path) = path {
            for entry in std::fs::read_dir(path)
                .expect("Failed to get crr dir")
                .flatten()
            {
                let path = entry.path();
                println!(
                    "{:?} {:?} {:?}",
                    entry.file_name(),
                    Kinds::classifiy_kinds(&path, &path.metadata().unwrap()),
                    Kinds::is_hidden(&path)
                );
            }
        }
    }
}
