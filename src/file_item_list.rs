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

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::Kinds;

    #[test]
    fn is_this_hidden_files() {
        let v = ["/dir/.dir/", "/.text.txt", ".config", ".../.../", "..."];

        for name in v.into_iter() {
            assert!(Kinds::is_hidden(Path::new(name)));
        }
    }

    #[test]
    fn classifiy_file_kinds_test() {
        let v = [
            ("/sample.rs", Kinds::File(false)),
            ("sample.py", Kinds::File(false)),
            (".sample", Kinds::File(true)),
        ];
        for (name, kinds) in v.into_iter() {
            let path = Path::new(name);
            assert_eq!(Kinds::classifiy_kinds(path), kinds);
        }
    }

    #[test]
    fn classifiy_dir_kinds_test() {
        todo!();
    }
}
