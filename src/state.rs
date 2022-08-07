use std::path::{Path, PathBuf};

use crate::file_item_list::directory_item::Directory;
use crate::file_item_list::file_item::FileItem;
use crate::file_item_list::Kinds;
use crate::path_process::{make_file_items_from_dirpath, pathbuf_to_string_name};
use tui::widgets::TableState;

#[derive(Debug, Clone)]
pub struct StatefulDirectory {
    directory: Directory,
    file_items: Vec<FileItem>,
    length: usize,
    state: TableState,
}

impl StatefulDirectory {
    pub fn new(dir_path: PathBuf, is_show: bool) -> StatefulDirectory {
        let file_items = if is_show {
            make_file_items_from_dirpath(&dir_path)
        } else {
            make_file_items_from_dirpath(&dir_path)
                .into_iter()
                .filter(|item| item.kinds() != Kinds::File(true))
                .filter(|item| item.kinds() != Kinds::Directory(true))
                .collect::<Vec<FileItem>>()
        };
        StatefulDirectory {
            directory: Directory::new(dir_path),
            state: TableState::default(),
            length: file_items.len(),
            file_items,
        }
    }

    pub fn push_file_item_and_sort(&mut self, item: FileItem) {
        if self.contain_name(item.name_ref()) {
            return;
        }
        self.file_items.push(item);
        self.length = self.file_items.len();
        let index = self.file_items.len() - 1;
        self.state.select(Some(index));
        self.sort_file_items_by_kinds();
    }

    pub fn directory(&self) -> &Directory {
        &self.directory
    }

    pub fn dir_path(&self) -> &Path {
        self.directory.pathbuf()
    }

    pub fn _dir_parent_path(&self) -> &PathBuf {
        self.directory.parent()
    }

    pub fn dir_name(&self) -> String {
        self.directory.name().to_owned()
    }

    pub fn file_items(&self) -> &Vec<FileItem> {
        &self.file_items
    }

    pub fn state_table(&self) -> TableState {
        self.state.clone()
    }

    pub fn select_file_item_by_index(&mut self, i: Option<usize>) {
        self.state.select(i);
    }

    pub fn get_selected_file_item(&self) -> Option<&FileItem> {
        if let Some(i) = self.state.selected() {
            self.file_items.get(i)
        } else {
            None
        }
    }

    pub fn select_bottom_file_item(&mut self) {
        if self.length < 1 {
            return;
        }

        self.state.select(Some(self.length - 1));
    }

    pub fn select_top_file_item(&mut self) {
        if self.length < 1 {
            return;
        }
        self.state.select(Some(0));
    }

    pub fn select_next_file_item(&mut self) {
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

    pub fn select_previous_file_item(&mut self) {
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

    pub fn sort_file_items_by_kinds(&mut self) {
        self.file_items
            .sort_by(|a, b| b.kinds().partial_cmp(&a.kinds()).unwrap());
    }

    pub fn remove_file_item_with_path(&mut self, path: &Path) {
        if let Some(i) = self.file_items().iter().position(|x| x.path() == path) {
            self.file_items.remove(i);
        }
    }

    pub fn _remove_file_item_with_name(&mut self, name: &str) {
        if let Some(i) = self
            .file_items()
            .iter()
            .map(|x| pathbuf_to_string_name(x.path()))
            .position(|n| n.as_str() == name)
        {
            self.file_items.remove(i);
        }
    }

    pub fn contain_name(&self, name: &str) -> bool {
        self.file_items.iter().any(|x| x.name_ref() == name)
    }

    pub fn _contain_path(&self, path: &Path) -> bool {
        self.file_items.iter().any(|p| p.path() == path)
    }
}
