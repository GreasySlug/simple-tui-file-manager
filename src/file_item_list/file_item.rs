use super::Kinds;
use chrono::{DateTime, Utc};
use regex::Regex;
use std::{
    fs::Metadata,
    ops::Range,
    path::{Path, PathBuf},
};

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

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn meta(&self) -> &Metadata {
        &self.meta
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn name_ref(&self) -> &str {
        &self.name
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

    pub fn is_hidden_file(&self) -> bool {
        self.kinds() == Kinds::File(true)
    }

    pub fn is_hidden_dir(&self) -> bool {
        self.kinds() == Kinds::Directory(true)
    }

    pub fn find(&self, pattern: &Regex) -> Option<(&str, Range<usize>)> {
        pattern.find(&self.name).map(|m| (m.as_str(), m.range()))
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
