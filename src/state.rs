use std::path::PathBuf;

use crate::file_item_list::directory_item::Directory;
use crate::file_item_list::file_item::FileItem;
use crate::path_process::{make_info_files_from_dirpath, pathbuf_to_string_name};
use tui::widgets::TableState;

#[derive(Debug, Clone)]
pub struct StatefulDirectory {
    directory: Directory,
    file_items: Vec<FileItem>,
    length: usize,
    state: TableState,
}

impl StatefulDirectory {
    pub fn new(dir_path: PathBuf) -> StatefulDirectory {
        let file_item = make_info_files_from_dirpath(&dir_path);
        StatefulDirectory {
            directory: Directory::new(dir_path),
            length: file_item.len(),
            file_items: file_item,
            state: TableState::default(),
        }
    }

    pub fn take_crr_dir_name(&self) -> String {
        let path = self.directory.get_path();
        pathbuf_to_string_name(&path)
    }

    pub fn take_file_items(&self) -> Vec<FileItem> {
        self.file_items.clone()
    }

    pub fn take_state_dir(&self) -> TableState {
        self.state.clone()
    }

    pub fn select_top(&mut self) {
        if self.length < 1 {
            return;
        }
        self.state.select(Some(0));
    }

    pub fn select_next(&mut self) {
        if self.length < 1 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.file_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn select_previous(&mut self) {
        if self.length < 1 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.file_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn is_selected(&self) -> bool {
        self.state.selected().is_some()
    }

    pub fn sort_by_kinds(&mut self) {
        self.file_items
            .sort_by(|a, b| b.kinds().partial_cmp(&a.kinds()).unwrap());
    }
}
