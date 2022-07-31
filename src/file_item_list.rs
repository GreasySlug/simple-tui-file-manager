use std::{fs::Metadata, path::Path};

use crate::path_process::pathbuf_to_string_name;

pub mod directory_item;
pub mod file_item;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Extension {
    C,
    CPlusPlus,
    CSharp,
    Go,
    Java,
    JavaScript,
    Markdown,
    Rust,
    Ruby,
    Python,
    Perl,
    Toml,
    Unknwon,
}

impl Extension {
    pub fn classify_extension(path: &Path) -> Self {
        match path.extension() {
            None => Self::Unknwon,
            Some(extension) => {
                let extension = extension.to_str();
                match extension {
                    Some("c") => Self::C,
                    Some("cpp") => Self::CPlusPlus,
                    Some("cs") => Self::CSharp,
                    Some("go") => Self::Go,
                    Some("java") => Self::Java,
                    Some("js") => Self::JavaScript,
                    Some("md") => Self::Markdown,
                    Some("pl") => Self::Perl,
                    Some("py") => Self::Python,
                    Some("rb") => Self::Ruby,
                    Some("rs") => Self::Rust,
                    Some("toml") => Self::Toml,
                    _ => Self::Unknwon,
                }
            }
        }
    }
}

enum DirKinds {
    GitIgnore,
    Git,
    Vscode,
    Examples,
    Src,
    Config,
    Home,
    Profile,
    Documents,
    Downloads,
    Desktop,
    Share,
    Images,
    Unknwon,
}

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

    use super::Extension;

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

    #[test]
    fn test_type_extension() {
        let files = [
            ("sample.py", Extension::Python),
            ("sample.rb", Extension::Ruby),
            ("sample.c", Extension::C),
            ("sample.cs", Extension::CSharp),
            ("sample.cpp", Extension::CPlusPlus),
            ("sample.go", Extension::Go),
            ("sample.java", Extension::Java),
            ("sample.js", Extension::JavaScript),
            ("sample.md", Extension::Markdown),
            ("sample.pl", Extension::Perl),
            ("sample.rs", Extension::Rust),
            ("sample.toml", Extension::Toml),
            ("aaa", Extension::Unknwon),
        ];
        for (filename, types) in files.iter() {
            let path = Path::new(filename);
            let path_ex = Extension::classify_extension(path);
            assert_eq!(path_ex, *types);
        }
    }
}
