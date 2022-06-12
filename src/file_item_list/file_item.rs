use super::Kinds;
use chrono::{DateTime, Utc};
use std::{
    fs::Metadata,
    path::{Path, PathBuf},
};

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

#[derive(Debug, Clone)]
pub struct FileItem {
    name: String,
    path: PathBuf,
    meta: Metadata,
    kinds: Kinds,
    extension: Option<Extension>,
}

impl FileItem {
    pub fn new(
        name: String,
        path: PathBuf,
        meta: Metadata,
        kinds: Kinds,
        extension: Option<Extension>,
    ) -> Self {
        Self {
            name,
            path,
            meta,
            kinds,
            extension,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn meta(&self) -> &Metadata {
        &self.meta
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn kinds(&self) -> Kinds {
        self.kinds.clone()
    }

    pub fn get_file_item_size(&self) -> String {
        let size = self.meta.len();
        calc_file_item_size(size)
    }

    // true: write and read, false: only read
    pub fn get_permission(&self) -> bool {
        let perm = self.meta.permissions();
        perm.readonly()
    }

    pub fn get_created_date_and_time(&self) -> String {
        let time = self.meta.created();
        if time.is_err() {
            return "-".to_string();
        }

        let created_time: DateTime<Utc> = time.unwrap().into();
        created_time.format("%F-%R").to_string()
    }
}

const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
const DECIMAL_PLACE: f64 = 100.0;
fn calc_file_item_size(byte: u64) -> String {
    if byte < 1 {
        return format!("{:>5}", "-");
    }
    let byte = byte as f64;
    let i = byte.log(1024.0).round();
    let size = byte / 1024.0f64.powf(i) * DECIMAL_PLACE;
    format!("{:>5} {}", size.round() / DECIMAL_PLACE, UNITS[i as usize])
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::Extension;

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
